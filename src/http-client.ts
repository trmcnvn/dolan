import { Context, Effect, Layer, Schema } from "effect";

const REQUEST_TIMEOUT_MS = 8_000;

export class HttpError extends Schema.TaggedError<HttpError>()("HttpError", {
  message: Schema.String,
  url: Schema.String,
}) {}

export class HttpClient extends Context.Service<
  HttpClient,
  {
    readonly getText: (
      url: string,
      userAgent: string,
    ) => Effect.Effect<string, HttpError>;
  }
>()("dolan/HttpClient") {
  static readonly layer = Layer.succeed(
    HttpClient,
    HttpClient.of({
      getText: Effect.fn("HttpClient.getText")(function* (
        url: string,
        userAgent: string,
      ): Effect.fn.Return<string, HttpError> {
        return yield* Effect.tryPromise({
          try: async () => {
            const response = await fetch(url, {
              headers: { "user-agent": userAgent },
              signal: AbortSignal.timeout(REQUEST_TIMEOUT_MS),
            });

            if (!response.ok) {
              throw new Error(`${response.status} ${response.statusText}`);
            }

            return response.text();
          },
          catch: (error) =>
            new HttpError({
              message: error instanceof Error ? error.message : "Request failed",
              url,
            }),
        });
      }),
    }),
  );
}
