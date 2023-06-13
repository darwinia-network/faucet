use crate::errors::BackendResult;
use crate::state::AppState;
use crate::types::form::FaucetReceiveForm;

pub struct FaucetService;

impl FaucetService {
  pub async fn receive(state: AppState, form: FaucetReceiveForm) -> BackendResult<String> {
    let config = &state.config;

    Ok("http://pangolin.subscan.io".to_string())
  }
}
