/// <reference types="bun" />

import { describe, expect, test } from "bun:test";
import { Effect, Schema } from "effect";
import { DiscordInteractionSchema } from "./discord.ts";
import { verifyDiscordRequest } from "./signature.ts";

const bytesToHex = (value: ArrayBuffer) =>
  Array.from(new Uint8Array(value), (byte) => byte.toString(16).padStart(2, "0")).join(
    "",
  );

const signedRequest = async (body: string, timestamp: string) => {
  const keys = await crypto.subtle.generateKey({ name: "Ed25519" }, true, [
    "sign",
    "verify",
  ]);
  const publicKey = await crypto.subtle.exportKey("raw", keys.publicKey);
  const message = new TextEncoder().encode(`${timestamp}${body}`);
  const signature = await crypto.subtle.sign("Ed25519", keys.privateKey, message);

  return {
    body,
    publicKey: bytesToHex(publicKey),
    signature: bytesToHex(signature),
    timestamp,
  };
};

describe("Discord request verification", () => {
  test("accepts a current valid signature", async () => {
    const now = Date.now();
    const timestamp = `${Math.floor(now / 1_000)}`;
    const request = await signedRequest('{"type":1}', timestamp);

    const verified = await Effect.runPromise(verifyDiscordRequest({ ...request, now }));

    expect(verified).toBe(true);
  });

  test("rejects stale signed requests", async () => {
    const now = Date.now();
    const timestamp = `${Math.floor(now / 1_000) - 301}`;
    const request = await signedRequest('{"type":1}', timestamp);

    const verified = await Effect.runPromise(verifyDiscordRequest({ ...request, now }));

    expect(verified).toBe(false);
  });

  test("rejects invalid interaction payloads", async () => {
    const result = await Effect.runPromise(
      Effect.result(Schema.decodeUnknownEffect(DiscordInteractionSchema)('{"type":2}')),
    );

    expect(result._tag).toBe("Failure");
  });
});
