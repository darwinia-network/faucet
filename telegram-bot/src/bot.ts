import yargs from "yargs";
import Grammer from "./grammer";
// import Grammer from "./brain"

const cmdBot: yargs.CommandModule = {
  builder: (argv: yargs.Argv) => {
    return argv.positional("token", {
      alias: "k",
      describe: "the telegram-bot token",
      default: "",
      type: "string",
    }).positional("config", {
      alias: "c",
      describe: "the path of grammer.yml",
      default: "",
      type: "string",
    });
  },
  command: "bot [token] [config]",
  describe: "start darwinia telegram bot",
  handler: async (args: yargs.Arguments) => {
    const g = await Grammer.new((args.config as string));
    g.run((args.key as string));
  }
}

export default cmdBot;
