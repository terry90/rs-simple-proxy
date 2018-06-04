use hyper::{Body, Error, Request, Response};
use proxy::error::MiddlewareError;

pub enum MiddlewareResult {
  RespondWith(Response<Body>),
  Next,
}

use self::MiddlewareResult::Next;

pub trait Middleware {
  fn get_name(&self) -> &String;

  fn before_request(
    &mut self,
    _req: &mut Request<Body>,
    _req_id: u64,
  ) -> Result<MiddlewareResult, MiddlewareError> {
    Ok(Next)
  }

  fn after_request(&mut self, _req_id: u64) -> Result<MiddlewareResult, MiddlewareError> {
    Ok(Next)
  }

  fn request_failure(
    &mut self,
    _err: &Error,
    _req_id: u64,
  ) -> Result<MiddlewareResult, MiddlewareError> {
    Ok(Next)
  }

  fn request_success(
    &mut self,
    _req: &mut Response<Body>,
    _req_id: u64,
  ) -> Result<MiddlewareResult, MiddlewareError> {
    Ok(Next)
  }
}
