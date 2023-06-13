use actix_web::web;

use crate::route;

/// beetle router
pub struct FaucetRouter;

impl FaucetRouter {
  pub fn router(cfg: &mut web::ServiceConfig) {
    Self::middleware_generic(cfg);
    Self::route_generic(cfg);
    Self::route_v1(cfg);
  }
}

/// generic router
impl FaucetRouter {
  fn route_generic(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(route::generic::index));
  }

  fn middleware_generic(_cfg: &mut web::ServiceConfig) {}
}

impl FaucetRouter {
  fn route_v1(cfg: &mut web::ServiceConfig) {
    let scope_v1 = web::scope("/api/v1");
    let scope_v1_route = scope_v1
      .route("/hello", web::get().to(route::generic::index))
      .route("/faucet/receive", web::post().to(route::v1::faucet::receive))
      // just segment
      ;
    cfg.service(scope_v1_route);
  }
}
