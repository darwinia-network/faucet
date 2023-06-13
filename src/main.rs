use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::http::{header, StatusCode};
use actix_web::middleware::ErrorHandlers;
use actix_web::{App, HttpServer};
use strawberry_common::envh;
use strawberry_common::types::RunningMode;
use strawberry_config::types::StrawberryConfig;
use strawberry_initializer::AppInitializer;
use strawberry_state::state::AppStateBuilder;

use crate::router::FaucetRouter;

mod enhance;
mod external;
mod route;
mod router;

/// Start a server and use a `Router` to dispatch requests
#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
  let config = init_app().await?;
  if envh::running_mode() == RunningMode::Develop {
    tracing::debug!(target: "faucet", "CONFIG -> \n{:#?}", config);
  }
  init_sentry(&config).await?;
  start_server(config).await?;
  Ok(())
}

/// init app
async fn init_app() -> color_eyre::Result<StrawberryConfig> {
  Ok(AppInitializer.init()?)
}

/// init sentry
async fn init_sentry(config: &StrawberryConfig) -> color_eyre::Result<()> {
  let sentry_config = config.sentry.clone();
  if sentry_config.is_none() {
    return Ok(());
  }
  let sentry_config = sentry_config.unwrap();
  if sentry_config.dsn.is_empty() {
    return Err(color_eyre::eyre::eyre!("Missing sentry dsn"));
  }
  let _guard = sentry::init((
    sentry_config.dsn,
    sentry::ClientOptions {
      release: sentry::release_name!(),
      ..Default::default()
    },
  ));
  tracing::info!(target: "beetle-bin", "Sentry is initialized");
  Ok(())
}

/// start web server
async fn start_server(config: StrawberryConfig) -> color_eyre::Result<()> {
  let server_config = &config.server;
  let (host, port) = (server_config.host.clone(), server_config.port);
  tracing::info!(
    target: "beetle",
    "The beetle server listen {}:{}",
    &host,
    port,
  );
  let app_data = AppStateBuilder::new(config).app_state().await?;

  // Allow bursts with up to five requests per IP address
  // and replenishes one element every two seconds
  let governor_conf = GovernorConfigBuilder::default()
    .per_second(2)
    .burst_size(15)
    .finish()
    .unwrap();
  HttpServer::new(move || {
    App::new()
      .app_data(app_data.clone())
      .app_data(external::json_config())
      .app_data(external::query_config())
      .configure(FaucetRouter::router)
      .wrap(
        Cors::default()
          .send_wildcard()
          .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
          .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
          .allowed_header(header::CONTENT_TYPE)
          .supports_credentials()
          .max_age(3600),
      )
      .wrap(Governor::new(&governor_conf))
      .wrap(ErrorHandlers::new().handler(
        StatusCode::INTERNAL_SERVER_ERROR,
        enhance::middleware::error_handler::handle_500,
      ))
      .wrap(ErrorHandlers::new().handler(
        StatusCode::NOT_FOUND,
        enhance::middleware::error_handler::handle_404,
      ))
      .wrap(actix_web::middleware::Logger::default())
  })
  .bind((host, port))?
  .run()
  .await?;
  Ok(())
}
