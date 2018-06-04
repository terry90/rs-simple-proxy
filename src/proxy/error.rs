use hyper::{Body, Response, StatusCode};
use std::error::Error;

#[derive(Debug)]
pub struct MiddlewareError {
  pub details: String,
  pub body: String,
  pub status: StatusCode,
}

impl MiddlewareError {
  pub fn new(details: String, body: Option<String>, status: StatusCode) -> MiddlewareError {
    let body = match body {
      Some(body) => body,
      None => format!("Internal proxy server error: {}", &details),
    };

    MiddlewareError {
      details,
      status,
      body,
    }
  }

  pub fn to_json_response(&self) -> Response<Body> {
    Response::builder()
      .header("Content-Type", "application/json")
      .status(self.status)
      .body(Body::from(format!("{{\"error\":\"{}\"}}", self.body)))
      .unwrap()
  }
}

impl<E> From<E> for MiddlewareError
where
  E: Error,
{
  fn from(err: E) -> MiddlewareError {
    MiddlewareError::new(
      String::from(err.description()),
      None,
      StatusCode::INTERNAL_SERVER_ERROR,
    )
  }
}
