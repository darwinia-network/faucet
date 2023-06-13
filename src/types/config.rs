use strawberry_config::types::StrawberryConfig;

use serde::{Deserialize, Serialize};

/// App config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FaucetConfig {
  #[serde(flatten)]
  pub strawberry: StrawberryConfig,
  pub reservoir: Reservoir,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reservoir {
  pub private_key: String,
}
