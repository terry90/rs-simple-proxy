use hyper::{Body, Error, Request, Response};
use std::fmt::Debug;

use proxy::middleware::{Middleware, MiddlewareError};

#[derive(Clone)]
pub struct Routing<T>
where
  T: RoutingConfig,
{
  config: T,
}

#[derive(Debug, Clone)]
pub struct RoutingRule {
  pub from: String,
  pub to: String,
}

pub type RoutingRules = Vec<RoutingRule>;

pub trait RoutingConfig {
  fn get_routing_rules(&self) -> &RoutingRules;
}

impl<T: RoutingConfig> Middleware for Routing<T> {
  fn before_request(&mut self, req: &mut Request<Body>) -> Result<(), MiddlewareError> {
    debug!("before_request");
    Err(MiddlewareError::UnknownError)
  }

  fn after_request(&mut self) -> Result<(), MiddlewareError> {
    debug!("after_request");
    Err(MiddlewareError::UnknownError)
  }

  fn request_failure(&mut self, err: &Error) -> Result<(), MiddlewareError> {
    debug!("request_failure");
    Err(MiddlewareError::UnknownError)
  }

  fn request_success(&mut self, req: &mut Response<Body>) -> Result<(), MiddlewareError> {
    debug!("request_success");
    Err(MiddlewareError::UnknownError)
  }
}

impl<T> Routing<T>
where
  T: RoutingConfig + Debug,
{
  pub fn new(config: T) -> Self {
    info!("{:?}", config);
    Routing { config }
  }
}
