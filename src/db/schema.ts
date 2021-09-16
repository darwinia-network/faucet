export default abstract class BotDb {
  public abstract addAddr(addr: string): Promise<void>;

  public abstract hasReceived(addr: string): Promise<boolean>;

  public abstract nextDrop(id: number, interval: number): Promise<number>;

  public abstract lastDrop(id: number, last: number): Promise<void>;

  public abstract hasSupply(date: string, supply: number): Promise<boolean>;

  public abstract burnSupply(date: string, supply: number): Promise<void>;
}
