use chrono::{DateTime, Utc};
use hyper::{Body, Request, Response};
use std::collections::HashMap;

use proxy::middleware::{Middleware, MiddlewareError, MiddlewareResult, MiddlewareResult::Next};

#[derive(Clone, Default)]
pub struct Logger {
  start_time_queue: HashMap<u64, DateTime<Utc>>,
  name: String,
}

impl Middleware for Logger {
  fn get_name(&self) -> &String {
    &self.name
  }

  fn before_request(
    &mut self,
    req: &mut Request<Body>,
    req_id: u64,
  ) -> Result<MiddlewareResult, MiddlewareError> {
    info!(
      "[{}] Starting request to {}",
      &req_id.to_string()[..6],
      req.uri()
    );
    self.start_time_queue.insert(req_id, Utc::now());
    Ok(Next)
  }

  fn after_request(&mut self, req_id: u64) -> Result<MiddlewareResult, MiddlewareError> {
    let start_time = self.start_time_queue.remove(&req_id).unwrap(); // TODO avoid panic

    info!(
      "[{}] Request took {}ms",
      &req_id.to_string()[..6],
      (Utc::now() - start_time).num_milliseconds()
    );
    Ok(Next)
  }
}

impl Logger {
  pub fn new() -> Self {
    Logger {
      start_time_queue: HashMap::new(),
      name: String::from("Logger"),
    }
  }
}
