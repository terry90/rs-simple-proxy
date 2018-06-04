use http::uri::{Parts, Uri};
use hyper::header::HeaderValue;
use hyper::{Body, Request};
#[cfg(test)]
use mockers_derive::derive_mock;
use regex::Regex;
use std::fmt::Debug;

use proxy::middleware::{Middleware, MiddlewareError, MiddlewareResult, MiddlewareResult::Next};

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
  pub from: Regex, // TODO
  pub to: String,
}

pub type RouterRules = Vec<RouterRule>;

#[cfg_attr(test, derive_mock)]
pub trait RouterConfig {
  fn get_router_rules(&self) -> &RouterRules;
}

fn get_host(req: &mut Request<Body>) -> String {
  let uri = req.uri();

  match uri.host() {
    Some(host) => String::from(host),
    None => String::from(req.headers().get("host").unwrap().to_str().unwrap()), // TODO handle error
  }
}

fn inject_host(req: &mut Request<Body>, old_host: &str, host: &str) {
  {
    let headers = req.headers_mut();

    headers.insert("X-Forwarded-Host", HeaderValue::from_str(old_host).unwrap());
    headers.insert("host", HeaderValue::from_str(host).unwrap());
  }
  let mut parts = Parts::default();
  parts.scheme = Some("http".parse().unwrap());
  parts.authority = Some(host.parse().unwrap());

  if let Some(path_and_query) = req.uri().path_and_query() {
    parts.path_and_query = Some(path_and_query.clone());
  }

  debug!("{:?}", parts);

  *req.uri_mut() = Uri::from_parts(parts).unwrap();
}

impl<T: RouterConfig> Middleware for Router<T> {
  fn get_name(&self) -> &String {
    &self.name
  }

  fn before_request(
    &mut self,
    req: &mut Request<Body>,
    _req_id: u64,
  ) -> Result<MiddlewareResult, MiddlewareError> {
    let rules = self.config.get_router_rules();
  
    let host = get_host(req);
    debug!("Routing {}{}", host, req.uri());

    for ref rule in rules {
      debug!("Trying to convert {} to {}", &rule.from, &rule.to);
      let re = &rule.from;

      if re.is_match(&host) {
        let new_host = re.replace(&host, rule.to.as_str());
        // TODO ask @ workshop about security host etc..
        inject_host(req, &host, &new_host);
        break;
      }
    }

    Ok(Next)
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

#[cfg(test)]
mod tests {
  use super::{Router, RouterConfig};
  use mockers::Scenario;

  #[test]
  fn new_creates_new_router() {
    let scenario = Scenario::new();
    let config_mock = scenario.create_mock_for::<RouterConfig>();
    let router = Router::new(config_mock);

    assert_eq!("Router", router.name);
  }

  #[test]
  fn new_creates_new_router() {
    let scenario = Scenario::new();
    let config_mock = scenario.create_mock_for::<RouterConfig>();
    let router = Router::new(config_mock);

    assert_eq!("Router", router.name);
  }
}
