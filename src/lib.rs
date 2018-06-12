extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate http;
extern crate rand;
#[cfg(feature = "router")]
extern crate regex;
extern crate serde;
extern crate serde_json;

pub mod middlewares;
pub mod proxy;

use futures::future::Future;
use hyper::Server;
use std::fmt;
use std::sync::{Arc, Mutex};

use proxy::middleware::Middleware;
use proxy::service::ProxyServiceBuilder;

type Middlewares = Arc<Mutex<Vec<Box<Middleware + Send + Sync>>>>;

#[derive(Debug, Clone, Copy)]
pub enum Environment {
    Production,
    Staging,
    Development,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Environment::Production => write!(f, "production"),
            Environment::Staging => write!(f, "staging"),
            Environment::Development => write!(f, "development"),
        }
    }
}

impl std::str::FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "production" => Ok(Environment::Production),
            "staging" => Ok(Environment::Staging),
            "development" => Ok(Environment::Development),
            _ => Err({ String::from("valid values: production, staging, development") }),
        }
    }
}

pub struct SimpleProxy {
    port: u16,
    environment: Environment,
    middlewares: Middlewares,
}

impl SimpleProxy {
    pub fn new(port: u16, environment: Environment) -> Self {
        SimpleProxy {
            port,
            environment,
            middlewares: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn run(&self) {
        let addr = ([127, 0, 0, 1], self.port).into();

        let middlewares = self.middlewares.clone();
        let proxy = ProxyServiceBuilder::new(middlewares);

        info!("Running proxy in {} mode", self.environment);

        let server = Server::bind(&addr)
            .serve(proxy)
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }

    pub fn add_middleware(&mut self, middleware: Box<Middleware + Send + Sync>) {
        self.middlewares.lock().unwrap().push(middleware)
    }
}
