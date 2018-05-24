extern crate futures;
extern crate http;
extern crate hyper;

use futures::future;
use futures::future::IntoFuture;
use hyper::client::connect::HttpConnector;
use hyper::rt::Future;
use hyper::service::Service;
use hyper::{Body, Client, Request};
use proxy::middleware::Middleware;
use std::fmt::Debug;
use std::marker::Sync;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use Middlewares;

type BoxFut = Box<Future<Item = hyper::Response<Body>, Error = hyper::Error> + Send>;

pub struct ProxyService {
  client: Client<HttpConnector, Body>,
  middlewares: Middlewares,
}

fn convert_uri(uri: &hyper::Uri) -> hyper::Uri {
  let base: hyper::Uri = "http://localhost:4567".parse().unwrap();
  let mut parts: http::uri::Parts = base.into();
  if let Some(path_and_query) = uri.path_and_query() {
    parts.path_and_query = Some(path_and_query.clone());
  }

  hyper::Uri::from_parts(parts).unwrap() // Consider removing unwrap
}

fn convert_req<U: Debug>(base: hyper::Request<U>) -> hyper::Request<U> {
  let (mut parts, body) = base.into_parts();

  parts.uri = convert_uri(&parts.uri);

  let req = hyper::Request::from_parts(parts, body);

  req
}

impl Service for ProxyService {
  type Error = hyper::Error;
  type Future = BoxFut;
  type ReqBody = Body;
  type ResBody = Body;

  fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
    let time = Instant::now();
    let mut req = convert_req(req);

    let mws_failure = Arc::clone(&self.middlewares);
    let mws_success = Arc::clone(&self.middlewares);

    for mw in self.middlewares.lock().unwrap().iter_mut() {
      mw.before_request(&mut req);
    }

    let res = self
      .client
      .request(req)
      .map_err(move |err| {
        error!("{:?}", err);

        for mw in mws_failure.lock().unwrap().iter_mut() {
          mw.request_failure(&err);
        }
        for mw in mws_failure.lock().unwrap().iter_mut() {
          mw.after_request();
        }

        err
      })
      .map(move |mut res| {
        for mw in mws_success.lock().unwrap().iter_mut() {
          mw.request_success(&mut res);
        }
        for mw in mws_success.lock().unwrap().iter_mut() {
          mw.after_request();
        }
        res
      });

    Box::new(res)
  }
}

impl ProxyService {
  pub fn new(middlewares: Middlewares) -> Self {
    ProxyService {
      client: Client::new(),
      middlewares: middlewares,
    }
  }
}

impl IntoFuture for ProxyService {
  type Future = future::FutureResult<Self::Item, Self::Error>;
  type Item = Self;
  type Error = hyper::Error;

  fn into_future(self) -> Self::Future {
    future::ok(self)
  }
}
