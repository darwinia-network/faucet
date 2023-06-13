use actix_web::Responder;
use strawberry_state::state::AppState;

/// index route
pub async fn index(_state: AppState) -> impl Responder {
  "hello"
}
