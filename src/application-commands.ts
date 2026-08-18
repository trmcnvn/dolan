const ApplicationCommandOptionType = {
  String: 3,
  Integer: 4,
} as const;

export const applicationCommands = [
  {
    name: "ping",
    description: "Post the configured ping emoji.",
  },
  {
    name: "time",
    description: "Look up local times for one or more locations.",
    options: [
      {
        type: ApplicationCommandOptionType.String,
        name: "locations",
        description: "Up to 5 semicolon-separated locations, e.g. Melbourne; Kyiv",
        required: true,
      },
    ],
  },
  {
    name: "weather",
    description: "Show wttr.in weather for one or more locations.",
    options: [
      {
        type: ApplicationCommandOptionType.String,
        name: "locations",
        description: "Up to 5 semicolon-separated locations, e.g. Melbourne; Kyiv",
        required: true,
      },
      {
        type: ApplicationCommandOptionType.Integer,
        name: "style",
        description: "wttr.in view style, matching the old optional numeric argument.",
        required: false,
        min_value: 0,
      },
    ],
  },
] as const;
