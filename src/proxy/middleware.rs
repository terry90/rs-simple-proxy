use hyper::{Body, Error, Request, Response};

#[derive(Fail, Debug)]
pub enum MiddlewareError {
  #[fail(display = "An unknown error has occurred.")]
  UnknownError,
}

pub trait Middleware {
  fn get_name(&self) -> &String;

  fn before_request(
    &mut self,
    _req: &mut Request<Body>,
    _req_id: u64,
  ) -> Result<(), MiddlewareError> {
    Ok(())
  }

  fn after_request(&mut self, _req_id: u64) -> Result<(), MiddlewareError> {
    Ok(())
  }

  fn request_failure(&mut self, _err: &Error, _req_id: u64) -> Result<(), MiddlewareError> {
    Ok(())
  }

  fn request_success(
    &mut self,
    _req: &mut Response<Body>,
    _req_id: u64,
  ) -> Result<(), MiddlewareError> {
    Ok(())
  }
}
