use hyper::Request;

#[derive(Fail, Debug)]
enum MiddlewareError {
  #[fail(display = "An unknown error has occurred.")]
  UnknownError,
}

trait Middleware {
  fn before_request(req: Request) -> Result<Request, MiddlewareError>;
  fn after_request(req: Request) -> Result<Request, MiddlewareError>;
  fn request_failure(req: Request) -> Result<Request, MiddlewareError>;
  fn request_success(req: Request) -> Result<Request, MiddlewareError>;
}
