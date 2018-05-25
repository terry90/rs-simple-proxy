use hyper::{Body, Error, Request, Response};
use std::fmt::Debug;

use proxy::middleware::{Middleware, MiddlewareError};

#[derive(Clone)]
pub struct Router<T>
where
  T: RouterConfig,
{
  config: T,
  name: String,
}

#[derive(Debug, Clone)]
pub struct RouterRule {
  pub from: String,
  pub to: String,
}

pub type RouterRules = Vec<RouterRule>;

pub trait RouterConfig {
  fn get_router_rules(&self) -> &RouterRules;
}

impl<T: RouterConfig> Middleware for Router<T> {
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

impl<T> Router<T>
where
  T: RouterConfig + Debug,
{
  pub fn new(config: T) -> Self {
    Router {
      config,
      name: String::from("Router"),
    }
  }
}
