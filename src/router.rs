use actix_web::web;

use crate::route;

/// beetle router
pub struct FaucetRouter;

impl FaucetRouter {
  pub fn router(cfg: &mut web::ServiceConfig) {
    Self::middleware_generic(cfg);
    Self::route_generic(cfg);
  }
}

/// generic router
impl FaucetRouter {
  fn route_generic(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(route::generic::index));
  }

  fn middleware_generic(_cfg: &mut web::ServiceConfig) {}
}
