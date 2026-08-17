import { Option, Schema } from "effect";

export const InteractionType = {
  Ping: 1,
  ApplicationCommand: 2,
} as const;

export const InteractionResponseType = {
  Pong: 1,
  ChannelMessageWithSource: 4,
  DeferredChannelMessageWithSource: 5,
} as const;

export const MessageFlags = {
  Ephemeral: 64,
} as const;

const DiscordCommandOptionSchema = Schema.Struct({
  name: Schema.String,
  type: Schema.Number,
  value: Schema.optional(Schema.Union([Schema.String, Schema.Number, Schema.Boolean])),
});

export const DiscordInteractionSchema = Schema.fromJsonString(
  Schema.Struct({
    id: Schema.String,
    application_id: Schema.String,
    token: Schema.String,
    type: Schema.Number,
    data: Schema.optional(
      Schema.Struct({
        name: Schema.optional(Schema.String),
        options: Schema.optional(Schema.Array(DiscordCommandOptionSchema)),
      }),
    ),
  }),
);

export type DiscordInteraction = Schema.Schema.Type<typeof DiscordInteractionSchema>;

export type DiscordResponseData = {
  content?: string;
  embeds?: ReadonlyArray<{ image?: { url: string } }>;
  flags?: number;
};

export type DiscordInteractionResponse =
  | { type: typeof InteractionResponseType.Pong }
  | {
      type:
        | typeof InteractionResponseType.ChannelMessageWithSource
        | typeof InteractionResponseType.DeferredChannelMessageWithSource;
      data?: DiscordResponseData;
    };

export const jsonResponse = (
  body: DiscordInteractionResponse,
  init: ResponseInit = {},
) =>
  new Response(JSON.stringify(body), {
    ...init,
    headers: {
      "content-type": "application/json;charset=UTF-8",
      ...init.headers,
    },
  });

export const messageResponse = (
  data: DiscordResponseData,
): DiscordInteractionResponse => ({
  type: InteractionResponseType.ChannelMessageWithSource,
  data,
});

export const deferredResponse = (): DiscordInteractionResponse => ({
  type: InteractionResponseType.DeferredChannelMessageWithSource,
});

export const getStringOption = (
  interaction: DiscordInteraction,
  name: string,
): string | undefined => {
  const value = interaction.data?.options?.find(
    (option) => option.name === name,
  )?.value;
  return Option.getOrUndefined(Schema.decodeUnknownOption(Schema.String)(value));
};

export const getIntegerOption = (
  interaction: DiscordInteraction,
  name: string,
): number | undefined => {
  const value = interaction.data?.options?.find(
    (option) => option.name === name,
  )?.value;
  return Option.getOrUndefined(Schema.decodeUnknownOption(Schema.Int)(value));
};

export const editOriginalInteractionResponse = async (
  interaction: DiscordInteraction,
  data: DiscordResponseData,
) => {
  const response = await fetch(
    `https://discord.com/api/v10/webhooks/${interaction.application_id}/${interaction.token}/messages/@original`,
    {
      method: "PATCH",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(data),
    },
  );

  if (!response.ok) {
    throw new Error(
      `Discord follow-up failed: ${response.status} ${await response.text()}`,
    );
  }
};
