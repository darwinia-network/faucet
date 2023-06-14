use actix_web::http::{header::ContentType, StatusCode};
use actix_web::HttpResponse;
use ethers::prelude::ProviderError;
use strawberry_actix_web::types::Resp;
use thiserror::Error as ThisError;

pub type BackendResult<T> = actix_web::Result<T, BackendError>;

/// Backend error
#[derive(ThisError, Debug)]
pub enum BackendError {
  #[error("Custom: {0}")]
  Custom(String),
  #[error("{0}")]
  Ethers(#[from] ProviderError),
  #[error("{0}")]
  Json(#[from] serde_json::Error),
}

impl actix_web::error::ResponseError for BackendError {
  fn error_response(&self) -> HttpResponse {
    let msg = format!("{}", self);
    let resp = Resp::err_with_message(msg);
    HttpResponse::build(self.status_code())
      .insert_header(ContentType::json())
      .body(resp.to_json_or_err())
  }

  fn status_code(&self) -> StatusCode {
    StatusCode::BAD_REQUEST
  }
}
