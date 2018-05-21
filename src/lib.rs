extern crate dotenv;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate clap;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub mod proxy;

use dotenv::dotenv;
use futures::future::Future;
use hyper::Server;
use std::fmt;

use proxy::service::ProxyService;

#[derive(Debug)]
pub enum Environment {
    Production,
    Staging,
    Development,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Production => write!(f, "production"),
            Staging => write!(f, "staging"),
            Development => write!(f, "development"),
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
}

impl SimpleProxy {
    pub fn new(port: u16, environment: Environment) -> Self {
        SimpleProxy { port, environment }
    }

    pub fn run(&self) {
        let addr = ([127, 0, 0, 1], self.port).into();

        let proxy = || ProxyService::new();

        info!("Running proxy in {} mode", self.environment);

        let server = Server::bind(&addr)
            .serve(proxy)
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }
}
