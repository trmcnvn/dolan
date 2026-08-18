import { Effect, Schema } from "effect";
import { DomUtils, parseDocument } from "htmlparser2";
import {
  type DiscordInteraction,
  type DiscordResponseData,
  getIntegerOption,
  getStringOption,
} from "./discord.ts";
import { HttpClient, HttpError } from "./http-client.ts";

const DISCORD_CONTENT_LIMIT = 2_000;
const MAX_LOCATIONS = 5;

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

export const splitLocations = (value: string) =>
  value
    .split(";")
    .map((part) => part.trim())
    .filter(Boolean);

export class TimeParseError extends Schema.TaggedError<TimeParseError>()(
  "TimeParseError",
  {
    location: Schema.String,
  },
) {}

export const extractTime = Effect.fn("extractTime")(function* (
  html: string,
  location: string,
): Effect.fn.Return<string, TimeParseError> {
  const document = parseDocument(html);
  const time = DomUtils.findOne((element) => element.name === "time", document);

  if (time === null) {
    return yield* new TimeParseError({ location });
  }

  return DomUtils.textContent(time).trim();
});

const weatherImageUrl = (location: string, style: number) =>
  `https://wttr.in/${encodeURIComponent(location)}_m${style}Fq_lang=en.png`;

const tooManyLocations = (locations: ReadonlyArray<string>) =>
  locations.length > MAX_LOCATIONS
    ? { content: `Use at most ${MAX_LOCATIONS} locations per command.` }
    : undefined;

const handlePing = (config: CommandConfig): Effect.Effect<DiscordResponseData> =>
  Effect.succeed({ content: config.pingEmoji });

const getTime = Effect.fn("getTime")(function* (
  http: HttpClient["Service"],
  location: string,
): Effect.fn.Return<string, HttpError | TimeParseError> {
  const html = yield* http.getText(
    `https://time.is/${encodeURIComponent(location)}`,
    "Dolan/1.0",
  );
  const time = yield* extractTime(html, location);
  return `${location}: ${time}`;
});

const handleTime = Effect.fn("handleTime")(function* (
  interaction: DiscordInteraction,
): Effect.fn.Return<DiscordResponseData, HttpError | TimeParseError, HttpClient> {
  const locations = splitLocations(getStringOption(interaction, "locations") ?? "");

  if (locations.length === 0) {
    return { content: "Give me at least one location." };
  }

  const limitResponse = tooManyLocations(locations);
  if (limitResponse !== undefined) {
    return limitResponse;
  }

  const http = yield* HttpClient;
  const times = yield* Effect.forEach(
    locations,
    (location) => getTime(http, location),
    { concurrency: "unbounded" },
  );

  return { content: codeBlock(times.join("\n")) };
});

const getWeather = Effect.fn("getWeather")(function* (
  http: HttpClient["Service"],
  location: string,
  style: number,
): Effect.fn.Return<{ readonly location: string; readonly text: string }, HttpError> {
  const text = yield* http.getText(
    `https://wttr.in/${encodeURIComponent(location)}?m${style}FqT&lang=en`,
    "curl",
  );
  return { location, text };
});

const handleWeather = Effect.fn("handleWeather")(function* (
  interaction: DiscordInteraction,
): Effect.fn.Return<DiscordResponseData, HttpError, HttpClient> {
  const locations = splitLocations(getStringOption(interaction, "locations") ?? "");
  const style = Math.max(0, getIntegerOption(interaction, "style") ?? 0);

  if (locations.length === 0) {
    return { content: "Give me at least one location." };
  }

  const limitResponse = tooManyLocations(locations);
  if (limitResponse !== undefined) {
    return limitResponse;
  }

  const http = yield* HttpClient;
  const weather = yield* Effect.forEach(
    locations,
    (location) => getWeather(http, location, style),
    { concurrency: "unbounded" },
  );
  const content: Array<string> = [];
  const embeds: Array<{ image: { url: string } }> = [];

  for (const { location, text } of weather) {
    if (text.trim().length === 0) {
      continue;
    }

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

  return response.content === undefined && response.embeds === undefined
    ? { content: "No weather found." }
    : response;
});

export const handleCommand = Effect.fn("handleCommand")(function* (
  interaction: DiscordInteraction,
  config: CommandConfig,
): Effect.fn.Return<DiscordResponseData, HttpError | TimeParseError, HttpClient> {
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
