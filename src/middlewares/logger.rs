use chrono::{DateTime, Utc};
use hyper::{Body, Request};
use serde_json;

use proxy::error::MiddlewareError;
use proxy::middleware::{Middleware, MiddlewareResult, MiddlewareResult::Next};
use proxy::service::State;

#[derive(Clone, Default)]
pub struct Logger;

/// # Panics
/// May panic if the request state has not been initialized in `before_request`.
/// e.g If a middleware responded early before the logger in `before_request`.
impl Middleware for Logger {
  fn name() -> String {
    String::from("Logger")
  }

  fn before_request(
    &mut self,
    req: &mut Request<Body>,
    req_id: u64,
    state: &State,
  ) -> Result<MiddlewareResult, MiddlewareError> {
    info!(
      "[{}] Starting request to {}",
      &req_id.to_string()[..6],
      req.uri()
    );
    let now = serde_json::to_string(&Utc::now()).expect("[Logger] Cannot serialize DateTime");
    self.set_state(req_id, state, now)?;
    Ok(Next)
  }

  fn after_request(
    &mut self,
    req_id: u64,
    state: &State,
  ) -> Result<MiddlewareResult, MiddlewareError> {
    let start_time = self.get_state(req_id, state)?;
    match start_time {
      Some(time) => {
        let start_time: DateTime<Utc> = serde_json::from_str(&time)?;

        info!(
          "[{}] Request took {}ms",
          &req_id.to_string()[..6],
          (Utc::now() - start_time).num_milliseconds()
        );
      }
      None => error!("[Logger] start time not found in state"),
    }
    Ok(Next)
  }
}

impl Logger {
  pub fn new() -> Self {
    Logger {}
  }
}
