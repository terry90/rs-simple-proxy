use hyper::{Body, Request};

#[derive(Fail, Debug)]
pub enum MiddlewareError {
  #[fail(display = "An unknown error has occurred.")]
  UnknownError,
}

pub trait Middleware {
  fn before_request(req: Request<Body>) -> Result<Request<Body>, MiddlewareError>;
  fn after_request(req: Request<Body>) -> Result<Request<Body>, MiddlewareError>;
  fn request_failure(req: Request<Body>) -> Result<Request<Body>, MiddlewareError>;
  fn request_success(req: Request<Body>) -> Result<Request<Body>, MiddlewareError>;
}
