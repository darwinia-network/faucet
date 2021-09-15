import child_process from "child_process";
import fs from "fs";
import os from "os";
import path from "path";
import prompts from "prompts";
import rawCj from "./static/config.json";
import rawTj from "./static/types.json";

export interface IConfig {
  node: string;
  seed: string;
}

/**
 * darwinia.js config
 *
 * @property {String} node - darwinia node address
 * @property {String} seed - darwinia account seed
 */
export class Config {
  static warn(config: Config) {
    if (config.node === "") {
      console.error("darwinia node has not been configured");
      process.exit(0);
    }
  }

  /// Load and merge config from file
  static load(p: string, defaultConfig: Record<string, any>): Record<string, any> {
    let json: Record<string, any> = defaultConfig;
    if (!fs.existsSync(p)) {
      fs.writeFileSync(p, JSON.stringify(json, null, 2));
    } else {
      const cur = Object.assign(json, JSON.parse(fs.readFileSync(p, "utf8")));
      fs.writeFileSync(p, JSON.stringify(cur, null, 2));
      json = cur;
    }

    return json
  }

  public node: string;
  public path: string;
  public types: Record<string, any>;
  private seed: string;

  constructor() {
    const home = os.homedir();
    const root = path.resolve(home, ".darwinia/faucet");
    const conf = path.resolve(root, "config.json");
    const types = path.resolve(root, "types.json");

    // Init pathes
    this.path = conf;

    // Check root
    if (!fs.existsSync(root)) {
      fs.mkdirSync(root, {recursive: true});
    }

    const cj = Config.load(conf, rawCj);
    this.node = cj.node;
    this.seed = cj.seed;
    this.types = Config.load(types, rawTj);

    // Warn config
    Config.warn(this);
  }

  /**
   * Raise a prompt if seed not exists
   */
  public async checkSeed(): Promise<string> {
    if (this.seed !== "") {
      return this.seed;
    }

    const ans = await prompts({
      type: "text",
      name: "seed",
      message: "Please input your darwinia seed:",
    }, {
      onCancel: () => {
        console.error("You can fill the seed field in `~/.darwinia/config.json` manually");
        process.exit(0);
      }
    });

    const curConfig: IConfig = JSON.parse(fs.readFileSync(this.path, "utf8"));
    const seed = String(ans.seed).trim();
    curConfig.seed = seed;
    this.seed = seed;
    fs.writeFileSync(
      this.path,
      JSON.stringify(curConfig, null, 2)
    );

    return seed;
  }

  /**
   * edit dj.json
   */
  public async edit(): Promise<void> {
    child_process.spawnSync("vi", [this.path], {
      stdio: "inherit",
    });
  }
}
