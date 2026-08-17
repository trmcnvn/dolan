import { Effect } from "effect";

const hexToBytes = (hex: string) => {
  const normalized = hex.trim();
  if (normalized.length % 2 !== 0 || /[^0-9a-f]/iu.test(normalized)) {
    throw new Error("Invalid hex value");
  }

  const bytes = new Uint8Array(normalized.length / 2);
  for (let index = 0; index < normalized.length; index += 2) {
    bytes[index / 2] = Number.parseInt(normalized.slice(index, index + 2), 16);
  }
  return bytes;
};

const utf8ToBytes = (value: string) => new TextEncoder().encode(value);

const concatBytes = (left: Uint8Array, right: Uint8Array) => {
  const bytes = new Uint8Array(left.length + right.length);
  bytes.set(left);
  bytes.set(right, left.length);
  return bytes;
};

export const verifyDiscordRequest = Effect.fn("verifyDiscordRequest")(function* ({
  body,
  publicKey,
  signature,
  timestamp,
}: {
  body: string;
  publicKey: string;
  signature: string;
  timestamp: string;
}): Effect.fn.Return<boolean> {
  return yield* Effect.promise(async () => {
    try {
      const publicKeyBytes = hexToBytes(publicKey);
      const signatureBytes = hexToBytes(signature);
      const message = concatBytes(utf8ToBytes(timestamp), utf8ToBytes(body));

      for (const name of ["Ed25519", "ed25519"]) {
        try {
          const key = await crypto.subtle.importKey(
            "raw",
            publicKeyBytes,
            { name },
            false,
            ["verify"],
          );

          return await crypto.subtle.verify({ name }, key, signatureBytes, message);
        } catch {
          // Try the next accepted spelling.
        }
      }
    } catch {
      return false;
    }

    return false;
  });
});
