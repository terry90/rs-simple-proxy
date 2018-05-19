extern crate dotenv;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate clap;
extern crate failure;
#[macro_use]
extern crate failure_derive;
mod config;
mod proxy;

use dotenv::dotenv;
use futures::future::Future;
use hyper::Server;

use config::Config;
use proxy::service::ProxyService;

fn run(config: &Config) {
    let addr = ([127, 0, 0, 1], config.port).into();

    let proxy = || ProxyService::new();

    let server = Server::bind(&addr)
        .serve(proxy)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
