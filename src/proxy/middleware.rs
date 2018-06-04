use hyper::{Body, Error, Request, Response};

#[derive(Fail, Debug, PartialEq)]
pub enum MiddlewareError {
  #[fail(display = "An unknown error has occurred.")]
  UnknownError,
}

#[derive(Debug)]
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
    _resp: &mut Response<Body>,
    _req_id: u64,
  ) -> Result<MiddlewareResult, MiddlewareError> {
    Ok(Next)
  }
}

#[cfg(test)]
mod tests {
  use super::Middleware;
  use super::MiddlewareResult;
  use hyper::{Body, Request, Response};

  struct FakeMiddleware {
    name: String,
  }

  impl Middleware for FakeMiddleware {
    fn get_name(&self) -> &String {
      &self.name
    }
  }

  #[test]
  fn before_request_returns_ok_next() {
    let mut middleware = FakeMiddleware {
      name: String::from("Fake Middleware"),
    };

    match middleware.before_request(&mut Request::new(Body::empty()), 0) {
      Ok(MiddlewareResult::Next) => (),
      x => panic!("before_request returned {:?} instead of Ok(Next)", x),
    }
  }

  #[test]
  fn after_request_returns_ok_next() {
    let mut middleware = FakeMiddleware {
      name: String::from("Fake Middleware"),
    };

    match middleware.after_request(0) {
      Ok(MiddlewareResult::Next) => (),
      x => panic!("after_request returned {:?} instead of Ok(Next)", x),
    }
  }

  // #[test]
  // fn request_failure_returns_ok() {
  //   let mut middleware = FakeMiddleware {
  //     name: String::from("Fake Middleware"),
  //   };

  //   assert_eq!(middleware.request_failure(Error::new_closed(), 0), Ok(()));
  // }

  #[test]
  fn request_success_returns_ok_next() {
    let mut middleware = FakeMiddleware {
      name: String::from("Fake Middleware"),
    };

    match middleware.request_success(&mut Response::new(Body::empty()), 0) {
      Ok(MiddlewareResult::Next) => (),
      x => panic!("request_success returned {:?} instead of Ok(Next)", x),
    }
  }
}
