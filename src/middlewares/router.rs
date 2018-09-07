use http::uri::{Parts, Uri};
use hyper::header::HeaderValue;
use hyper::{Body, Request, StatusCode};
use regex::Regex;
use std::fmt::Debug;

use proxy::error::MiddlewareError;
use proxy::middleware::MiddlewareResult::Next;
use proxy::middleware::{Middleware, MiddlewareResult};
use proxy::service::State;

use serde_json;

#[derive(Clone)]
pub struct Router<T> {
    config: T,
    name: String,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub from: Regex, // TODO
    pub to: String,
    pub public: bool,
}

pub type RouterRules = Vec<Route>;

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchedRoute {
    pub uri: String,
    pub public: bool,
}

pub trait RouterConfig {
    fn get_router_rules(&self) -> &RouterRules;
}

fn get_host(req: &mut Request<Body>) -> Result<String, MiddlewareError> {
    let uri = req.uri();

    match uri.host() {
        Some(host) => Ok(String::from(host)),
        None => Ok(String::from(req.headers().get("host").unwrap().to_str()?)), // TODO handle error
    }
}

fn inject_host(req: &mut Request<Body>, old_host: &str, host: &str) -> Result<(), MiddlewareError> {
    {
        let headers = req.headers_mut();

        headers.insert("X-Forwarded-Host", HeaderValue::from_str(old_host).unwrap());
        headers.insert("host", HeaderValue::from_str(host).unwrap());
    }
    let mut parts = Parts::default();
    parts.scheme = Some("http".parse()?);
    parts.authority = Some(host.parse()?);

    if let Some(path_and_query) = req.uri().path_and_query() {
        parts.path_and_query = Some(path_and_query.clone());
    }

    debug!("Found a route to {:?}", parts);

    *req.uri_mut() = Uri::from_parts(parts)?;

    Ok(())
}

impl<T: RouterConfig> Middleware for Router<T> {
    fn name() -> String {
        String::from("Router")
    }

    fn before_request(
        &mut self,
        req: &mut Request<Body>,
        req_id: u64,
        state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        let routes = self.config.get_router_rules();

        let host = get_host(req)?;
        debug!("Routing => Host: {} URI: {}", host, req.uri());

        for route in routes {
            debug!("Trying to convert {} to {}", &route.from, &route.to);
            let re = &route.from;

            if re.is_match(&host) {
                let new_host = re.replace(&host, route.to.as_str());
                debug!("Proxying to {}", &new_host);
                inject_host(req, &host, &new_host)?;
                self.set_state(
                    req_id,
                    state,
                    serde_json::to_string(&MatchedRoute {
                        uri: req.uri().to_string(),
                        public: route.public,
                    })?,
                )?;
                return Ok(Next);
            }
        }

        Err(MiddlewareError::new(
            String::from("No route matched"),
            Some(String::from("Not found")),
            StatusCode::NOT_FOUND,
        ))
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
