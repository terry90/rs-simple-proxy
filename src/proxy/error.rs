use hyper::{Body, Response, StatusCode};
use std::error::Error;

#[derive(Debug)]
pub struct MiddlewareError {
  pub description: String,
  pub body: String,
  pub status: StatusCode,
}

impl From<MiddlewareError> for Response<Body> {
  fn from(err: MiddlewareError) -> Response<Body> {
    err.to_json_response()
  }
}

impl MiddlewareError {
  pub fn new(description: String, body: Option<String>, status: StatusCode) -> MiddlewareError {
    let body = match body {
      Some(body) => body,
      None => format!("Internal proxy server error: {}", &description),
    };

    debug!("Middleware error: {}", &description);

    MiddlewareError {
      description,
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
