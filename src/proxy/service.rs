extern crate futures;
extern crate http;
extern crate hyper;

use futures::future;
use futures::future::IntoFuture;
use hyper::client::connect::HttpConnector;
use hyper::rt::Future;
use hyper::service::Service;
use hyper::{Body, Client, Request, Response};
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

    // Create references for future callbacks
    // references are moved in each chained future (map,then..)
    let mws_failure = Arc::clone(&self.middlewares);
    let mws_success = Arc::clone(&self.middlewares);
    let mws_after = Arc::clone(&self.middlewares);

    let req_id = self.rng.next_u64();

    let mut before_res: Option<Response<Body>> = None;
    for mw in self.middlewares.lock().unwrap().iter_mut() {
      // Run all middlewares->before_request
      if let Some(res) = match mw.before_request(&mut req, req_id) {
        Err(err) => Some(Response::from(err)),
        Ok(RespondWith(response)) => Some(response),
        Ok(Next) => None,
      } {
        // Stop when an early response is wanted
        before_res = Some(res);
        break;
      }
    }

    if let Some(res) = before_res {
      return Box::new(future::ok(self.early_response(req_id, res)));
    }

    let res = self
      .client
      .request(req)
      .map_err(move |err| {
        for mw in mws_failure.lock().unwrap().iter_mut() {
          // TODO: think about graceful handling
          if let Err(err) = mw.request_failure(&err, req_id) {
            error!("[{}] request_failure errored: {:?}", mw.get_name(), err);
          }
        }
        err
      })
      .map(move |mut res| {
        for mw in mws_success.lock().unwrap().iter_mut() {
          match mw.request_success(&mut res, req_id) {
            Err(err) => res = Response::from(err),
            Ok(RespondWith(response)) => res = response,
            Ok(Next) => (),
          }
        }
        res
      })
      .then(move |mut res| {
        for mw in mws_after.lock().unwrap().iter_mut() {
          match mw.after_request(req_id) {
            Err(err) => res = Ok(Response::from(err)),
            Ok(RespondWith(response)) => res = Ok(response),
            Ok(Next) => (),
          }
        }
        res
      });

    Box::new(res)
  }
}

impl ProxyService {
  fn early_response(&self, req_id: u64, mut res: Response<Body>) -> Response<Body> {
    for mw in self.middlewares.lock().unwrap().iter_mut() {
      match mw.after_request(req_id) {
        Err(err) => res = Response::from(err),
        Ok(RespondWith(response)) => res = response,
        Ok(Next) => (),
      }
    }
    debug!("Early response is {:?}", &res);
    res
  }

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
