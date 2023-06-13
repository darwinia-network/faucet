use actix_web::http::header;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{dev, Result};

pub fn handle_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
  res.response_mut().headers_mut().insert(
    header::CONTENT_TYPE,
    header::HeaderValue::from_static("application/json"),
  );
  Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

pub fn handle_404<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
  res.response_mut().headers_mut().insert(
    header::CONTENT_TYPE,
    header::HeaderValue::from_static("application/json"),
  );
  Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}
