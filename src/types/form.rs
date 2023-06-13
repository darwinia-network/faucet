use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FaucetReceiveForm {
  pub chain: String,
  pub address: String,
  pub user: String,
}
