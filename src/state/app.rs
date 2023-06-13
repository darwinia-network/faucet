use actix_web::web;

use crate::types::config::FaucetConfig;

pub type AppState = web::Data<FaucetState>;

pub struct FaucetState {
  pub config: FaucetConfig,
}

impl FaucetState {
  /// Create app state
  fn app_state(self) -> AppState {
    web::Data::new(self)
  }
}

/// app state builder
#[derive(Clone, Debug)]
pub struct AppStateBuilder {
  config: FaucetConfig,
}

impl AppStateBuilder {
  /// create new builder instance
  pub fn new(config: FaucetConfig) -> Self {
    Self { config }
  }
}

impl AppStateBuilder {
  /// get app state
  pub async fn app_state(&self) -> AppState {
    let state = FaucetState {
      config: self.config.clone(),
    };
    state.app_state()
  }
}
