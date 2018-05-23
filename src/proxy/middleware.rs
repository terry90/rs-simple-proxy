use hyper::{Body, Error, Request, Response};

#[derive(Fail, Debug)]
pub enum MiddlewareError {
  #[fail(display = "An unknown error has occurred.")]
  UnknownError,
}

pub trait Middleware {
  fn before_request(&mut self, req: &mut Request<Body>) -> Result<(), MiddlewareError>;
  fn after_request(&mut self) -> Result<(), MiddlewareError>;
  fn request_failure(&mut self, err: &Error) -> Result<(), MiddlewareError>;
  fn request_success(&mut self, req: &mut Response<Body>) -> Result<(), MiddlewareError>;
}
