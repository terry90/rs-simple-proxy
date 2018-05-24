use hyper::{Body, Error, Request, Response};
use std::time::Instant;

use proxy::middleware::{Middleware, MiddlewareError};

#[derive(Clone)]
pub struct Logging {
  pub started_at: Instant,
}

impl Middleware for Logging {
  fn before_request(&mut self, req: &mut Request<Body>) -> Result<(), MiddlewareError> {
    debug!("Starting request to {}", req.uri());
    self.started_at = Instant::now();
    Ok(())
  }

  fn after_request(&mut self) -> Result<(), MiddlewareError> {
    Ok(())
  }

  fn request_failure(&mut self, err: &Error) -> Result<(), MiddlewareError> {
    Ok(())
  }

  fn request_success(&mut self, req: &mut Response<Body>) -> Result<(), MiddlewareError> {
    let duration = Instant::now().duration_since(self.started_at);

    let subsec_diff = f64::from(duration.subsec_nanos()) / f64::from(1_000_000);
    let diff = subsec_diff + f64::from(duration.as_secs() as u32) * f64::from(1000);

    let diff = format!("{:.3}", diff);

    info!("Request took {}ms", diff);
    Ok(())
  }
}

impl Logging {
  pub fn new() -> Self {
    warn!("New instance of logging mw");
    Logging {
      started_at: Instant::now(),
    }
  }
}
