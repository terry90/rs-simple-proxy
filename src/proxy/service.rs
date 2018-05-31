extern crate futures;
extern crate http;
extern crate hyper;

use futures::future;
use futures::future::IntoFuture;
use hyper::client::connect::HttpConnector;
use hyper::rt::Future;
use hyper::service::Service;
use hyper::{Body, Client, Request};
use std::fmt::Debug;
use std::sync::Arc;

use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::FromEntropy;

use proxy::middleware::MiddlewareResult::*;
use Middlewares;

type BoxFut = Box<Future<Item = hyper::Response<Body>, Error = hyper::Error> + Send>;

pub struct ProxyService {
  client: Client<HttpConnector, Body>,
  middlewares: Middlewares,
  rng: SmallRng,
}

impl Service for ProxyService {
  type Error = hyper::Error;
  type Future = BoxFut;
  type ReqBody = Body;
  type ResBody = Body;

  fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
    let (parts, body) = req.into_parts();
    let mut req = Request::from_parts(parts, body);

    let mws_failure = Arc::clone(&self.middlewares);
    let mws_success = Arc::clone(&self.middlewares);

    let req_id = self.rng.next_u64();

    for mw in self.middlewares.lock().unwrap().iter_mut() {
      match mw.before_request(&mut req, req_id) {
        Err(err) => error!("[{}] before_request errored: {:?}", mw.get_name(), err),
        Ok(RespondWith(response)) => return Box::new(future::ok(response)),
        Ok(Next) => (),
      }
    }

    let res = self
      .client
      .request(req)
      .map_err(move |err| {
        for mw in mws_failure.lock().unwrap().iter_mut() {
          match mw.request_failure(&err, req_id) {
            Err(err) => error!("[{}] request_failure errored: {:?}", mw.get_name(), err),
            // Ok(RespondWith(response)) => Box::new(response),
            _ => (),
            // Ok(Next) => (),
          }
        }
        for mw in mws_failure.lock().unwrap().iter_mut() {
          match mw.after_request(req_id) {
            Err(err) => error!("[{}] after_request errored: {:?}", mw.get_name(), err),
            // Ok(RespondWith(response)) => Box::new(response),
            _ => (),
            // Ok(Next) => (),
          }
        }
        err
      })
      .map(move |mut res| {
        for mw in mws_success.lock().unwrap().iter_mut() {
          match mw.request_success(&mut res, req_id) {
            Err(err) => error!("[{}] request_success errored: {:?}", mw.get_name(), err),
            Ok(RespondWith(response)) => return response,
            Ok(Next) => (),
          }
        }
        for mw in mws_success.lock().unwrap().iter_mut() {
          match mw.after_request(req_id) {
            Err(err) => error!("[{}] after_request errored: {:?}", mw.get_name(), err),
            Ok(RespondWith(response)) => return response,
            Ok(Next) => (),
          }
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
      rng: SmallRng::from_entropy(),
      middlewares,
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
