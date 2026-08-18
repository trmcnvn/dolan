/// <reference types="bun" />

import { describe, expect, test } from "bun:test";
import { Effect, Layer } from "effect";
import {
  DiscordClient,
  type DiscordInteraction,
  type DiscordResponseData,
} from "./discord.ts";
import { HttpClient, HttpError } from "./http-client.ts";
import { handleDeferredCommand } from "./worker.ts";

const interaction: DiscordInteraction = {
  id: "interaction-id",
  application_id: "application-id",
  token: "interaction-token",
  type: 2,
  data: {
    name: "time",
    options: [{ name: "locations", type: 3, value: "UTC" }],
  },
};

describe("deferred commands", () => {
  test("edits the original Discord response", async () => {
    const edits: Array<DiscordResponseData> = [];
    const httpLayer = Layer.succeed(
      HttpClient,
      HttpClient.of({ getText: () => Effect.succeed("<time>12:00</time>") }),
    );
    const discordLayer = Layer.succeed(
      DiscordClient,
      DiscordClient.of({
        editOriginalResponse: (_interaction, data) =>
          Effect.sync(() => {
            edits.push(data);
          }),
      }),
    );

    await Effect.runPromise(
      handleDeferredCommand(interaction, { pingEmoji: "🏓" }).pipe(
        Effect.provide(Layer.merge(httpLayer, discordLayer)),
      ),
    );

    expect(edits).toEqual([{ content: "```\nUTC: 12:00\n```" }]);
  });

  test("turns upstream failures into a Discord response", async () => {
    const edits: Array<DiscordResponseData> = [];
    const httpLayer = Layer.succeed(
      HttpClient,
      HttpClient.of({
        getText: (url) => Effect.fail(new HttpError({ message: "timed out", url })),
      }),
    );
    const discordLayer = Layer.succeed(
      DiscordClient,
      DiscordClient.of({
        editOriginalResponse: (_interaction, data) =>
          Effect.sync(() => {
            edits.push(data);
          }),
      }),
    );

    await Effect.runPromise(
      handleDeferredCommand(interaction, { pingEmoji: "🏓" }).pipe(
        Effect.provide(Layer.merge(httpLayer, discordLayer)),
      ),
    );

    expect(edits).toEqual([
      {
        content: "The upstream service failed for https://time.is/UTC: timed out",
      },
    ]);
  });
});
