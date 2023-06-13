use actix_web::Responder;

use crate::state::AppState;

/// index route
pub async fn index(_state: AppState) -> impl Responder {
  "hello"
}
