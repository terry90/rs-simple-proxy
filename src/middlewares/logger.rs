use chrono::{DateTime, Utc};
use hyper::{Body, Request, Response};
use serde_json;

use crate::proxy::error::MiddlewareError;
use crate::proxy::middleware::MiddlewareResult::Next;
use crate::proxy::middleware::{Middleware, MiddlewareResult};
use crate::proxy::service::{ServiceContext, State};

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
        context: &ServiceContext,
        state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        info!(
            "[{}] Starting a {} request to {}",
            &context.req_id.to_string()[..6],
            req.method(),
            req.uri()
        );
        let now = serde_json::to_string(&Utc::now()).expect("[Logger] Cannot serialize DateTime");
        self.set_state(context.req_id, state, now)?;
        Ok(Next)
    }

    fn after_request(
        &mut self,
        _res: Option<&mut Response<Body>>,
        context: &ServiceContext,
        state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        let start_time = self.get_state(context.req_id, state)?;
        match start_time {
            Some(time) => {
                let start_time: DateTime<Utc> = serde_json::from_str(&time)?;

                info!(
                    "[{}] Request took {}ms",
                    &context.req_id.to_string()[..6],
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
