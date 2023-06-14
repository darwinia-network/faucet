use actix_web::{web, HttpRequest, Responder};
use strawberry_actix_web::types::Resp;

use crate::services::faucet::FaucetService;
use crate::state::AppState;
use crate::types::form::FaucetReceiveForm;

/// modify a vendor
pub async fn receive(
  _req: HttpRequest,
  state: AppState,
  data: web::Json<FaucetReceiveForm>,
) -> actix_web::Result<impl Responder> {
  let config = &state.config;
  let form = data.0;
  let data = FaucetService::receive(state, form).await?;
  Ok(web::Json(Resp::ok_with_data(data)))
}
