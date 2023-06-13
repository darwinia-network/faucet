use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use strawberry_actix_web::types::Resp;

fn common_response(origin: &str, message: String, code: StatusCode) -> actix_web::error::Error {
  let resp = Resp::err_with_message(&message);
  let body = resp.to_json_or_err();
  let cause = format!("[{}] [{}] {}", origin, code, message);
  tracing::error!(target: "beetle-bin", "Error in response: {}", cause);
  actix_web::error::InternalError::from_response(
    cause,
    HttpResponse::build(code)
      .content_type("application/json")
      .body(body),
  )
  .into()
}

pub fn json_config() -> web::JsonConfig {
  web::JsonConfig::default().error_handler(|err, _req| {
    let code = err.status_code();
    let origin = "payload-json";
    let message = format!("[{}] {}", origin, err);
    common_response(origin, message, code)
  })
}

pub fn query_config() -> web::QueryConfig {
  web::QueryConfig::default().error_handler(|err, _req| {
    let code = err.status_code();
    let origin = "payload-query";
    let message = format!("[{}] {}", origin, err);
    common_response(origin, message, code)
  })
}
