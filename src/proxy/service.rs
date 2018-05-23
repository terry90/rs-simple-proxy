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

type BoxFut = Box<Future<Item = hyper::Response<Body>, Error = hyper::Error> + Send>;

pub struct ProxyService<T: 'static>
where
  T: Middleware + Send + Sync + Clone,
{
  client: Client<HttpConnector, Body>,
  middlewares: Arc<Mutex<Vec<T>>>,
}

fn convert_uri(uri: &hyper::Uri) -> hyper::Uri {
  let base: hyper::Uri = "http://localhost:3000".parse().unwrap();
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

impl<T: 'static> Service for ProxyService<T>
where
  T: Middleware + Send + Sync + Clone,
{
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
        let diff =
          f64::from(Instant::now().duration_since(time).subsec_nanos()) / f64::from(1_000_000);
        let diff = format!("{:.3}", diff);
        info!("Request took {}ms", diff);

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

impl<T: 'static> ProxyService<T>
where
  T: Middleware + Send + Sync + Clone,
{
  pub fn new(middlewares: Vec<T>) -> Self {
    ProxyService {
      client: Client::new(),
      middlewares: Arc::new(Mutex::new(middlewares)),
    }
  }
}

impl<T: 'static> IntoFuture for ProxyService<T>
where
  T: Middleware + Send + Sync + Clone,
{
  type Future = future::FutureResult<Self::Item, Self::Error>;
  type Item = Self;
  type Error = hyper::Error;

  fn into_future(self) -> Self::Future {
    future::ok(self)
  }
}
