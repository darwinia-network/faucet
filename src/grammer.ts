import fs from "fs";
import os from "os";
import path from "path";
import {ApiPromise, HttpProvider} from "@polkadot/api";
import {Keyring} from "@polkadot/keyring";
import {KeyringPair} from "@polkadot/keyring/types";
import {cryptoWaitReady} from "@polkadot/util-crypto";
import * as yml from "js-yaml";
import TelegramBot from "node-telegram-bot-api"
import {Config} from "./config";
import {BotDb, JDb, RDb} from "./db";

/**
 * Constants
 */
const LOCKER = path.resolve(os.tmpdir(), "faucet.lock");
const STATIC_GRAMMER_CONFIG: string = path.resolve(__dirname, "static/grammer.yml");

/**
 * fault grammers - the config below will generate from `grammer.yml`
 *
 * @param succeed string - the succeed grammer, contains ${hash} to replace
 * @param interval string - response when req account has requested fault in current interval
 * @param supply string - the current supply of today has run out, wait for next day
 * @param address string - wrong address alert
 */
export interface IFaucetGrammers {
  only: string;
  invite: string;
  invalid: string;
  empty: string;
  prefix: string;
  length: string;
  failed: string;
  succeed: string;
  interval: string;
  supply: string;
  address: string;
  received: string;
  config: ICommandFaucetConfig;
}

export interface ICommandFaucetConfig {
  supply: number;
  amount: number;
  interval: number;
}

export interface IGrammer {
  help: string;
  docs: string;
  book: string;
  dev: string;
  talk: string;
  more: string;
  about: string;
  faucet: IFaucetGrammers;
}


/**
 * this is the interface of Grammer service
 */
export interface IGrammerConfig {
  account: KeyringPair;
  api: ApiPromise;
  grammer: IGrammer;
  db: BotDb;
}

export default class Grammer {

  private grammer: IGrammer;
  private db: BotDb;
  private api: ApiPromise;
  private account: KeyringPair;

  constructor(conf: IGrammerConfig) {
    this.account = conf.account;
    this.api = conf.api;
    this.grammer = conf.grammer;
    this.db = conf.db;
  }

  static async new(
    grammerConfig: string = STATIC_GRAMMER_CONFIG,
    rdb: boolean = false,
    port: number = 6379,
    host: string = "127.0.0.1"
  ): Promise<Grammer> {
    await cryptoWaitReady();
    const config = new Config();
    const api = await ApiPromise.create({
      provider: new HttpProvider(config.node),
      types: config.types,
    });
    const account = new Keyring({
      type: "sr25519",
    }).addFromUri(
      await config.checkSeed(),
    );

    // revert config if path is empty
    if (grammerConfig === "") {
      grammerConfig = STATIC_GRAMMER_CONFIG;
    }

    // Check ENV
    if (process.env.DACTLE_REDIS_PORT) {
      port = Number.parseInt(process.env.DACTLE_REDIS_PORT, 10);
    }

    if (process.env.DACTLE_REDIS_HOST) {
      host = process.env.DACTLE_REDIS_HOST;
    }

    // Generate API
    const grammer: IGrammer = yml.load(fs.readFileSync(grammerConfig, "utf8")) as IGrammer;

    let db: BotDb;
    if (rdb) {
      db = new RDb(port, host);
    } else {
      db = new JDb(
        path.resolve(os.homedir(), ".darwinia/faucet/bot.json"),
        grammer.faucet.config.supply,
      );
    }

    return new Grammer({account, api, db, grammer});
  }

  private token(typedToken: string): string {
    if (!!typedToken) {
      return typedToken;
    }
    if (process.env.TELEGRAM_BOT_TOKEN) {
      return process.env.TELEGRAM_BOT_TOKEN;
    }
    console.error("The telegram-bot token is required.");
    process.exit(1);
  }

  /**
   * serve grammer with specfic key
   *
   * @param {string} token - telegram bot token
   */
  public async run(token: string) {
    // run locker
    this.locker();

    // start bot
    const botToken = this.token(token);
    const bot = new TelegramBot(botToken, {polling: true});
    bot.on("polling_error", (msg) => console.error(msg));
    bot.onText(/^\/\w+/, async (msg) => {
      if (msg.text === undefined) {
        return;
      }

      const match = msg.text.match(/\/\w+/);
      if (match === null) {
        return;
      }

      // reply
      const replayMsg = await this.reply(bot, msg, match[0].slice(1));
      if (replayMsg === undefined) {
        return;
      }
      const sentMsg = await bot.sendMessage(
        msg.chat.id,
        replayMsg,
        {
          reply_to_message_id: msg.message_id,
        }
      )

      // check if should delete message
      const that = this;
      if (sentMsg.text) {
        if (sentMsg.text === this.grammer.faucet.only.trim() ||
          sentMsg.text === this.grammer.faucet.invite.trim()) {
          await this.deleteMsg(bot, msg);
          setTimeout(async () => {
            await that.deleteMsg(bot, sentMsg);
          }, 30000);
        }
      }
    });
  }

  private async locker() {
    const that = this;
    setInterval(async () => {
      const balance = await that.api.query.system.account(this.account.address);
      if (Number.parseInt(balance.data.free.toString(), 10) < 1000 * 1000000000) {
        fs.writeFileSync(LOCKER, "");
      } else {
        if (fs.existsSync(LOCKER)) {
          fs.unlinkSync(LOCKER);
        }
      }
    }, 1000 * 30);
  }

  private async deleteMsg(bot: TelegramBot, msg: TelegramBot.Message): Promise<void> {
    await bot.deleteMessage(
      msg.chat.id, msg.message_id.toString(),
    ).catch((_: any) => {
      console.warn(`doesn't have the access for deleting messages`);
    });
  }

  private async reply(
    bot: TelegramBot,
    msg: TelegramBot.Message,
    cmd: string
  ): Promise<string | undefined> {
    switch (cmd) {
      case "faucet":
        if (fs.existsSync(LOCKER)) {
          return this.grammer.faucet.failed;
        } else {
          return await this.transfer(bot, msg);
        }
    }
  }

  /**
   * Transfer some ring wich multi-checks
   *
   * @param bot telegram bot instance
   * @param msg message
   */
  private async transfer(bot: TelegramBot, msg: TelegramBot.Message): Promise<string> {
    if (
      msg.text === undefined ||
      msg.from === undefined ||
      msg.from.id === undefined
    ) {
      return this.grammer.faucet.invalid;
    }

    // Get addr
    const matches = msg.text.match(/\/(\w+)\s+(\S+)/);
    if (matches === null || matches.length < 3) {
      return this.grammer.faucet.empty;
    }
    const addr = matches[2];

    // check supply
    const date = new Date().toJSON().slice(0, 10);
    const hasSupply = await this.db.hasSupply(date, this.grammer.faucet.config.supply);
    if (!hasSupply) {
      return this.grammer.faucet.supply;
    }

    // check user
    const nextDrop: number = await this.db.nextDrop(
      msg.from.id,
      this.grammer.faucet.config.interval
    );

    if (nextDrop > 0) {
      return this.grammer.faucet.interval.replace(
        "${hour}", Math.floor(nextDrop).toString()
      );
    }

    console.log(`${new Date()} trying to transfer to ${addr}`);
    if (addr.length !== 48) {
      return this.grammer.faucet.length;
    } else if (!addr.startsWith("2")) {
      return this.grammer.faucet.prefix;
    }

    // check addr
    const received: boolean = await this.db.hasReceived(addr);
    if (received) {
      console.trace(`${new Date()}: ${addr} has already received the airdrop`)
      return this.grammer.faucet.received;
    }

    // transfer to address
    bot.sendMessage(
      msg.chat.id,
      "Copy that! Well! Oh yes wait a minute mister postman!",
    );

    // check if tx failed
    let hash: string = "";
    try {
      /// Transfer to account
      const ex = this.api.tx.balances.transfer(
        addr, this.grammer.faucet.config.amount * 1000000000
      );
      await ex.signAndSend(this.account);
      hash = ex.hash.toString();
    } catch (err) {
      console.error(err);
      return this.grammer.faucet.failed;
    }

    // return exHash
    if (hash !== "") {
      await this.db.addAddr(addr);
      await this.db.burnSupply(date, this.grammer.faucet.config.supply);
      await this.db.lastDrop(msg.from.id, new Date().getTime())
      return this.grammer.faucet.succeed.replace("${hash}", hash);
    }
    return this.grammer.faucet.failed;
  }

}
