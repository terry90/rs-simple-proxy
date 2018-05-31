use hyper::{Body, Error, Request, Response};

#[derive(Fail, Debug, PartialEq)]
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
    _resp: &mut Response<Body>,
    _req_id: u64,
  ) -> Result<(), MiddlewareError> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::Middleware;
  use hyper::{Body, Error, Request, Response};

  struct FakeMiddleware {
    name: String,
  }

  impl Middleware for FakeMiddleware {
    fn get_name(&self) -> &String {
      &self.name
    }
  }

  #[test]
  fn before_request_returns_ok() {
    let mut middleware = FakeMiddleware {
      name: String::from("Fake Middleware"),
    };

    assert_eq!(
      middleware.before_request(&mut Request::new(Body::empty()), 0),
      Ok(())
    );
  }

  #[test]
  fn after_request_returns_ok() {
    let mut middleware = FakeMiddleware {
      name: String::from("Fake Middleware"),
    };

    assert_eq!(middleware.after_request(0), Ok(()));
  }

  // #[test]
  // fn request_failure_returns_ok() {
  //   let mut middleware = FakeMiddleware {
  //     name: String::from("Fake Middleware"),
  //   };

  //   assert_eq!(middleware.request_failure(Error::new_closed(), 0), Ok(()));
  // }

  #[test]
  fn request_success_returns_ok() {
    let mut middleware = FakeMiddleware {
      name: String::from("Fake Middleware"),
    };

    assert_eq!(
      Ok(()),
      middleware.request_success(&mut Response::new(Body::empty()), 0)
    );
  }
}
