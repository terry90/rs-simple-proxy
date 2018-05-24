use hyper::{Body, Error, Request, Response};
use std::fmt::Debug;

use proxy::middleware::{Middleware, MiddlewareError};

#[derive(Clone)]
pub struct Routing<T>
where
  T: RoutingConfig,
{
  config: T,
  name: String,
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
  fn get_name(&self) -> &String {
    &self.name
  }

  fn before_request(
    &mut self,
    _req: &mut Request<Body>,
    _req_id: u64,
  ) -> Result<(), MiddlewareError> {
    Err(MiddlewareError::UnknownError)
  }

  fn after_request(&mut self, _req_id: u64) -> Result<(), MiddlewareError> {
    Err(MiddlewareError::UnknownError)
  }

  fn request_failure(&mut self, _err: &Error, _req_id: u64) -> Result<(), MiddlewareError> {
    Err(MiddlewareError::UnknownError)
  }

  fn request_success(
    &mut self,
    _req: &mut Response<Body>,
    _req_id: u64,
  ) -> Result<(), MiddlewareError> {
    Err(MiddlewareError::UnknownError)
  }
}

impl<T> Routing<T>
where
  T: RoutingConfig + Debug,
{
  pub fn new(config: T) -> Self {
    Routing {
      config,
      name: String::from("Routing"),
    }
  }
}
