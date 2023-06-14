use ethers::prelude::{Http, Middleware, Provider, TransactionRequest};

use crate::errors::{BackendError, BackendResult};
use crate::state::AppState;
use crate::types::form::FaucetReceiveForm;

pub struct FaucetService;

impl FaucetService {
  pub async fn receive(state: AppState, form: FaucetReceiveForm) -> BackendResult<String> {
    let config = &state.config;
    let reservoir = &config.reservoir;
    let provider = Self::ether_provider(form.chain).await?;
    let accounts = provider.get_accounts().await?;
    // println!("{:?}", accounts);
    // let from = accounts[0];
    // let to = accounts[1];
    let from = "0xce5973975BF207582D480281c1355602e73Dca63";

    let tx = TransactionRequest::new()
      .to("0x5af9A1Be7bc22f9a6b2cE90acd69c23DCEEB23C2")
      .value(50)
      .from(array_bytes::hex2array(&reservoir.private_key).unwrap());

    let balance_before = provider.get_balance(from, None).await?;
    let nonce1 = provider.get_transaction_count(from, None).await?;

    // broadcast it via the eth_sendTransaction API
    let tx = provider.send_transaction(tx, None).await?.await?;

    println!("{}", serde_json::to_string(&tx)?);
    Ok("http://pangolin.subscan.io".to_string())
  }
}

impl FaucetService {
  async fn ether_provider(chain: String) -> BackendResult<Provider<Http>> {
    let chain = chain.to_lowercase();
    let endpoint = match &chain[..] {
      "pangolin" => "https://pangolin-rpc.darwinia.network",
      "pangoro" => "https://pangoro-rpc.darwinia.network",
      _ => return Err(BackendError::Custom("Not allowed chain".to_string())),
    };
    let provider =
      Provider::<Http>::try_from(endpoint).map_err(|e| BackendError::Custom(format!("{:?}", e)))?;
    Ok(provider)
  }
}
