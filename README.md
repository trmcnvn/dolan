![](https://proxy.duckduckgo.com/iu/?u=http%3A%2F%2Fi0.kym-cdn.com%2Fentries%2Ficons%2Foriginal%2F000%2F003%2F549%2FDolan.jpg&f=1)

## Dolan

Simple Discord bot for a personal Discord server, now deployed as a Cloudflare Worker using Discord slash-command interactions.

### Setup

```sh
bun install
cp .env.example .env
```

Fill in the Discord values in `.env`.

### Development

```sh
bun run check
bun run dev
```

Set the Discord interaction endpoint to the Worker URL, either `/` or `/interactions`.
Time and weather commands accept up to five semicolon-separated locations.

### Register slash commands

```sh
bun run register:commands
```

Set `DISCORD_GUILD_ID` for fast guild-scoped testing, or omit it for global commands.

### Deploy

```sh
bun run deploy
```

Alchemy handles the Cloudflare Worker infrastructure and binds `DISCORD_PUBLIC_KEY` plus optional `PING_EMOJI` from your environment.
