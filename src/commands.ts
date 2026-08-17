import { Effect, Schema } from "effect";
import {
  type DiscordInteraction,
  type DiscordResponseData,
  getIntegerOption,
  getStringOption,
} from "./discord.ts";

const DISCORD_CONTENT_LIMIT = 2_000;

export type CommandConfig = {
  pingEmoji: string;
};

const codeBlock = (text: string) => {
  const suffix = "\n```";
  const prefix = "```\n";
  const limit = DISCORD_CONTENT_LIMIT - prefix.length - suffix.length;
  const body = text.length > limit ? `${text.slice(0, limit - 1)}…` : text;
  return `${prefix}${body}${suffix}`;
};

const splitList = (value: string) =>
  value
    .split(";")
    .map((part) => part.trim())
    .filter(Boolean);

export class HttpError extends Schema.TaggedError<HttpError>()("HttpError", {
  message: Schema.String,
  url: Schema.String,
}) {}

const fetchText = Effect.fn("fetchText")(function* (
  url: string,
  userAgent: string,
): Effect.fn.Return<string, HttpError> {
  return yield* Effect.tryPromise({
    try: async () => {
      const response = await fetch(url, {
        headers: { "user-agent": userAgent },
        signal: AbortSignal.timeout(10_000),
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
});

const decodeHtml = (value: string) =>
  value
    .replaceAll("&amp;", "&")
    .replaceAll("&lt;", "<")
    .replaceAll("&gt;", ">")
    .replaceAll("&quot;", '"')
    .replaceAll("&#39;", "'");

const extractTime = (html: string) => {
  const match = /<time\b[^>]*>(.*?)<\/time>/isu.exec(html);
  return match ? decodeHtml(match[1].replace(/<[^>]+>/gu, "").trim()) : undefined;
};

const weatherText = (location: string, style: number) =>
  fetchText(
    `https://wttr.in/${encodeURIComponent(location)}?m${style}FqT&lang=en`,
    "curl",
  );

const weatherImageUrl = (location: string, style: number) =>
  `https://wttr.in/${encodeURIComponent(location)}_m${style}Fq_lang=en.png`;

const handlePing = (config: CommandConfig): Effect.Effect<DiscordResponseData> =>
  Effect.succeed({ content: config.pingEmoji });

const handleTime = Effect.fn("handleTime")(function* (
  interaction: DiscordInteraction,
): Effect.fn.Return<DiscordResponseData, HttpError> {
  const locations = splitList(getStringOption(interaction, "locations") ?? "");

  if (locations.length === 0) {
    return { content: "Give me at least one location." };
  }

  const times: Array<string> = [];
  for (const location of locations) {
    const html = yield* fetchText(
      `https://time.is/${encodeURIComponent(location)}`,
      "Dolan/1.0",
    );
    const time = extractTime(html);
    if (time) {
      times.push(`${location}: ${time}`);
    }
  }

  return {
    content: times.length > 0 ? codeBlock(times.join("\n")) : "No times found.",
  };
});

const handleWeather = Effect.fn("handleWeather")(function* (
  interaction: DiscordInteraction,
): Effect.fn.Return<DiscordResponseData, HttpError> {
  const locations = splitList(getStringOption(interaction, "locations") ?? "");
  const style = Math.max(0, getIntegerOption(interaction, "style") ?? 0);

  if (locations.length === 0) {
    return { content: "Give me at least one location." };
  }

  const content: Array<string> = [];
  const embeds: Array<{ image: { url: string } }> = [];

  for (const location of locations) {
    const text = yield* weatherText(location, style);
    if (text.length >= DISCORD_CONTENT_LIMIT - 10) {
      embeds.push({ image: { url: weatherImageUrl(location, style) } });
    } else {
      content.push(
        locations.length > 1 ? `${location}\n${codeBlock(text)}` : codeBlock(text),
      );
    }
  }

  const joined = content.join("\n");
  const response: DiscordResponseData = {};

  if (joined) {
    response.content = joined.slice(0, DISCORD_CONTENT_LIMIT);
  }

  if (embeds.length > 0) {
    response.embeds = embeds.slice(0, 10);
  }

  return response;
});

export const handleCommand = Effect.fn("handleCommand")(function* (
  interaction: DiscordInteraction,
  config: CommandConfig,
): Effect.fn.Return<DiscordResponseData, HttpError> {
  switch (interaction.data?.name) {
    case "ping":
      return yield* handlePing(config);
    case "time":
      return yield* handleTime(interaction);
    case "weather":
      return yield* handleWeather(interaction);
    default:
      return { content: "Unknown command." };
  }
});
