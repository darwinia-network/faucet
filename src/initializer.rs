use strawberry_initializer::error::{InitializeError, InitializeResult};
use strawberry_initializer::{config, logger};

use crate::types::config::FaucetConfig;

pub struct FaucetInitializer;

impl FaucetInitializer {
  pub fn init(&self) -> InitializeResult<FaucetConfig> {
    self.init_color_eyre()?;
    let config = self.init_config()?;
    self.init_logger(&config)?;
    Ok(config)
  }
}

impl FaucetInitializer {
  fn init_config(&self) -> InitializeResult<FaucetConfig> {
    config::init_config()
  }

  fn init_color_eyre(&self) -> InitializeResult<()> {
    color_eyre::install().map_err(|e| InitializeError::ColorEyre(format!("{e:?}")))
  }
  fn init_logger(&self, config: &FaucetConfig) -> InitializeResult<()> {
    logger::init_log(&config.strawberry)
  }
}
