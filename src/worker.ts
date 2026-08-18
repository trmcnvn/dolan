import { Effect, Layer, Schema } from "effect";
import type { WorkerEnv } from "../alchemy.run.ts";
import { type CommandConfig, handleCommand, type TimeParseError } from "./commands.ts";
import {
  deferredResponse,
  DiscordClient,
  DiscordInteractionSchema,
  type DiscordInteraction,
  InteractionResponseType,
  InteractionType,
  jsonResponse,
  messageResponse,
  MessageFlags,
} from "./discord.ts";
import { HttpClient, type HttpError } from "./http-client.ts";
import { verifyDiscordRequest } from "./signature.ts";

const RuntimeLive = Layer.merge(HttpClient.layer, DiscordClient.layer);

const unauthorized = () => new Response("Invalid signature", { status: 401 });
const notFound = () => new Response("Not found", { status: 404 });
const methodNotAllowed = () => new Response("Method not allowed", { status: 405 });

const parseInteraction = Schema.decodeUnknownEffect(DiscordInteractionSchema);

const commandConfig = (env: WorkerEnv): CommandConfig => ({
  pingEmoji: env.PING_EMOJI,
});

const commandErrorResponse = (error: HttpError | TimeParseError) => {
  switch (error._tag) {
    case "HttpError":
      return {
        content: `The upstream service failed for ${error.url}: ${error.message}`,
      };
    case "TimeParseError":
      return {
        content: `The time service returned an unexpected response for ${error.location}.`,
      };
  }
};

export const handleDeferredCommand = Effect.fn("handleDeferredCommand")(
  function* (interaction: DiscordInteraction, config: CommandConfig) {
    const discord = yield* DiscordClient;
    const data = yield* handleCommand(interaction, config).pipe(
      Effect.catch((error) => Effect.succeed(commandErrorResponse(error))),
    );
    yield* discord.editOriginalResponse(interaction, data);
  },
  Effect.catch((error) =>
    Effect.sync(() => console.error("Failed to edit Discord interaction", error)),
  ),
);

const handleInteraction = Effect.fn("handleInteraction")(
  function* (request: Request, env: WorkerEnv, context: ExecutionContext) {
    const signature = request.headers.get("x-signature-ed25519");
    const timestamp = request.headers.get("x-signature-timestamp");

    if (!signature || !timestamp) {
      return unauthorized();
    }

    const body = yield* Effect.tryPromise(() => request.text());
    const isValid = yield* verifyDiscordRequest({
      body,
      publicKey: env.DISCORD_PUBLIC_KEY,
      signature,
      timestamp,
    });

    if (!isValid) {
      return unauthorized();
    }

    const interaction = yield* parseInteraction(body);

    if (interaction.type === InteractionType.Ping) {
      return jsonResponse({ type: InteractionResponseType.Pong });
    }

    if (interaction.type !== InteractionType.ApplicationCommand) {
      return jsonResponse(
        messageResponse({
          content: "Unsupported interaction.",
          flags: MessageFlags.Ephemeral,
        }),
      );
    }

    if (interaction.data?.name === "ping") {
      const data = yield* handleCommand(interaction, commandConfig(env)).pipe(
        Effect.provide(HttpClient.layer),
      );
      return jsonResponse(messageResponse(data));
    }

    const deferred = handleDeferredCommand(interaction, commandConfig(env)).pipe(
      Effect.provide(RuntimeLive),
    );
    context.waitUntil(Effect.runPromise(deferred));
    return jsonResponse(deferredResponse());
  },
  Effect.catch((error) =>
    Effect.succeed(
      jsonResponse(
        messageResponse({
          content: `Sorry, that failed: ${String(error)}`,
          flags: MessageFlags.Ephemeral,
        }),
        { status: 200 },
      ),
    ),
  ),
);

const route = Effect.fn("route")(function* (
  request: Request,
  env: WorkerEnv,
  context: ExecutionContext,
): Effect.fn.Return<Response> {
  const url = new URL(request.url);

  if (request.method === "GET" && url.pathname === "/healthz") {
    return new Response("OK");
  }

  if (request.method !== "POST") {
    return url.pathname === "/" ? methodNotAllowed() : notFound();
  }

  if (url.pathname !== "/" && url.pathname !== "/interactions") {
    return notFound();
  }

  return yield* handleInteraction(request, env, context);
});

export default {
  fetch(request: Request, env: WorkerEnv, context: ExecutionContext) {
    return Effect.runPromise(route(request, env, context));
  },
};
