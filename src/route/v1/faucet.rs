use actix_web::{web, HttpRequest, Responder};

use crate::state::AppState;

/// modify a vendor
pub async fn receive(req: HttpRequest, _state: AppState) -> actix_web::Result<impl Responder> {
  Ok("hello")
}
