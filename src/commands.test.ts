/// <reference types="bun" />

import { describe, expect, test } from "bun:test";
import { Effect, Layer } from "effect";
import { handleCommand } from "./commands.ts";
import type { DiscordInteraction } from "./discord.ts";
import { HttpClient } from "./http-client.ts";

const interaction = (command: string, locations: string): DiscordInteraction => ({
  id: "interaction-id",
  application_id: "application-id",
  token: "interaction-token",
  type: 2,
  data: {
    name: command,
    options: [{ name: "locations", type: 3, value: locations }],
  },
});

const config = { pingEmoji: "🏓" };

describe("commands", () => {
  test("rejects more than five locations without making requests", async () => {
    const calls: Array<string> = [];
    const layer = Layer.succeed(
      HttpClient,
      HttpClient.of({
        getText: (url) =>
          Effect.sync(() => {
            calls.push(url);
            return "<time>12:00</time>";
          }),
      }),
    );

    const response = await Effect.runPromise(
      handleCommand(interaction("time", "a;b;c;d;e;f"), config).pipe(
        Effect.provide(layer),
      ),
    );

    expect(response.content).toBe("Use at most 5 locations per command.");
    expect(calls).toEqual([]);
  });

  test("looks up locations concurrently", async () => {
    let active = 0;
    let maximumActive = 0;
    const layer = Layer.succeed(
      HttpClient,
      HttpClient.of({
        getText: () =>
          Effect.promise(async () => {
            active += 1;
            maximumActive = Math.max(maximumActive, active);
            await Bun.sleep(20);
            active -= 1;
            return "<time>12:00</time>";
          }),
      }),
    );

    const response = await Effect.runPromise(
      handleCommand(interaction("time", "UTC;Melbourne;Kyiv"), config).pipe(
        Effect.provide(layer),
      ),
    );

    expect(maximumActive).toBe(3);
    expect(response.content).toContain("UTC: 12:00");
    expect(response.content).toContain("Melbourne: 12:00");
    expect(response.content).toContain("Kyiv: 12:00");
  });

  test("reports malformed time-service responses", async () => {
    const layer = Layer.succeed(
      HttpClient,
      HttpClient.of({ getText: () => Effect.succeed("missing time element") }),
    );

    const result = await Effect.runPromise(
      Effect.result(
        handleCommand(interaction("time", "UTC"), config).pipe(Effect.provide(layer)),
      ),
    );

    expect(result).toMatchObject({
      _tag: "Failure",
      failure: { _tag: "TimeParseError", location: "UTC" },
    });
  });

  test("returns a message when weather is empty", async () => {
    const layer = Layer.succeed(
      HttpClient,
      HttpClient.of({ getText: () => Effect.succeed("") }),
    );

    const response = await Effect.runPromise(
      handleCommand(interaction("weather", "UTC"), config).pipe(Effect.provide(layer)),
    );

    expect(response).toEqual({ content: "No weather found." });
  });
});
