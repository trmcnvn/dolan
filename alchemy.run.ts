import * as Alchemy from "alchemy";
import * as Cloudflare from "alchemy/Cloudflare";
import * as Config from "effect/Config";
import * as Effect from "effect/Effect";

export const Worker = Cloudflare.Worker("Dolan", {
  main: "./src/worker.ts",
  env: {
    DISCORD_PUBLIC_KEY: Config.string("DISCORD_PUBLIC_KEY"),
    PING_EMOJI: Config.string("PING_EMOJI").pipe(
      Config.withDefault("<:grey_question:582795707401109506>"),
    ),
  },
});

export type WorkerEnv = Cloudflare.InferEnv<typeof Worker>;

export default Alchemy.Stack(
  "Dolan",
  {
    providers: Cloudflare.providers(),
    state: Cloudflare.state(),
  },
  Effect.gen(function* () {
    const worker = yield* Worker;
    return { url: worker.url };
  }),
);
