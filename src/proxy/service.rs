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
use std::marker::Sync;
use std::time::Instant;

use proxy::middleware::Middleware;

type BoxFut = Box<Future<Item = hyper::Response<Body>, Error = hyper::Error> + Send>;

pub struct ProxyService<'a, T: 'a>
where
  T: Middleware<'a> + Sync + Send,
{
  client: Client<HttpConnector, Body>,
  middlewares: &'a Vec<T>,
}

fn convert_uri(uri: &hyper::Uri) -> hyper::Uri {
  let base: hyper::Uri = "http://localhost:3000".parse().unwrap();
  let mut parts: http::uri::Parts = base.into();
  if let Some(path_and_query) = uri.path_and_query() {
    parts.path_and_query = Some(path_and_query.clone());
  }

  hyper::Uri::from_parts(parts).unwrap() // Consider removing unwrap
}

fn convert_req<T: Debug>(base: hyper::Request<T>) -> hyper::Request<T> {
  let (mut parts, body) = base.into_parts();

  parts.uri = convert_uri(&parts.uri);

  let req = hyper::Request::from_parts(parts, body);

  println!("Req converted to {:?}", req);

  req
}

impl<'a, T> Service for ProxyService<'a, T>
where
  T: Middleware<'a> + Sync + Send,
{
  type Error = hyper::Error;
  type Future = BoxFut;
  type ReqBody = Body;
  type ResBody = Body;

  fn call(&'a mut self, req: Request<Self::ReqBody>) -> Self::Future {
    let time = Instant::now();
    println!("{:?}", req);
    let req = convert_req(req);

    let res = self
      .client
      .request(req)
      .map_err(move |err| {
        eprintln!("\n!!--!!\n{:?}\n!!--!!\n", err);
        if err.is_user() {};
        err
      })
      .map(move |res| {
        println!("\n--\n{:?}\n--\n", res);
        let diff =
          f64::from(Instant::now().duration_since(time).subsec_nanos()) / f64::from(1_000_000);
        let diff = format!("{:.3}", diff);
        println!("{}ms", diff);
        res
      });

    Box::new(res)
  }
}

impl<'a, T> ProxyService<'a, T>
where
  T: Middleware<'a> + Sync + Send,
{
  pub fn new(middlewares: &'a Vec<T>) -> Self {
    ProxyService {
      client: Client::new(),
      middlewares,
    }
  }
}

impl<'a, T> IntoFuture for ProxyService<'a, T>
where
  T: Middleware<'a> + Sync + Send,
{
  type Future = future::FutureResult<Self::Item, Self::Error>;
  type Item = Self;
  type Error = hyper::Error;

  fn into_future(self) -> Self::Future {
    future::ok(self)
  }
}
