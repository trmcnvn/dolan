/// <reference types="bun" />

import * as Effect from "effect/Effect";
import { applicationCommands } from "../src/application-commands.ts";

const env = Bun.env;

const requireEnv = (name: string) =>
  Effect.sync(() => {
    const value = env[name];
    if (!value) {
      throw new Error(`${name} is required`);
    }
    return value;
  });

const registerCommands = Effect.gen(function* () {
  const applicationId = yield* requireEnv("DISCORD_APPLICATION_ID");
  const botToken = yield* requireEnv("DISCORD_BOT_TOKEN");
  const guildId = env.DISCORD_GUILD_ID;

  const endpoint = guildId
    ? `https://discord.com/api/v10/applications/${applicationId}/guilds/${guildId}/commands`
    : `https://discord.com/api/v10/applications/${applicationId}/commands`;

  const response = yield* Effect.tryPromise(() =>
    fetch(endpoint, {
      method: "PUT",
      headers: {
        authorization: `Bot ${botToken}`,
        "content-type": "application/json",
      },
      body: JSON.stringify(applicationCommands),
    }),
  );

  if (!response.ok) {
    const body = yield* Effect.tryPromise(() => response.text());
    throw new Error(`Discord command registration failed: ${response.status} ${body}`);
  }

  const body = yield* Effect.tryPromise(() => response.json());
  console.log(
    `Registered ${Array.isArray(body) ? body.length : applicationCommands.length} command(s) ${
      guildId ? `for guild ${guildId}` : "globally"
    }.`,
  );
});

await Effect.runPromise(registerCommands);
