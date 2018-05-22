extern crate futures;
extern crate hyper;
// #[macro_use]
// extern crate clap;
// extern crate failure;
// #[macro_use]
// extern crate failure_derive;
#[macro_use]
extern crate log;

#[macro_use]
extern crate failure_derive;
extern crate failure;

pub mod proxy;

use futures::future::Future;
use hyper::Server;
use std::fmt;

use proxy::middleware::Middleware;
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

pub struct SimpleProxy<'a, T: 'a>
where
    T: Middleware<'a> + Sync + Send,
{
    port: u16,
    environment: Environment,
    middlewares: &'a Vec<T>,
}

impl<'a, T> SimpleProxy<'a, T>
where
    T: Middleware<'a> + Sync + Send,
{
    pub fn new(port: u16, environment: Environment) -> Self {
        SimpleProxy {
            port,
            environment,
            middlewares: &vec![], // 'a here
        }
    }

    pub fn run(&'a self) {
        let addr = ([127, 0, 0, 1], self.port).into();

        let proxy = || ProxyService::new(self.middlewares);

        info!("Running proxy in {} mode", self.environment);

        let server = Server::bind(&addr)
            .serve(proxy)
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::run(server);
    }

    pub fn add_middleware(&'a self, middleware: T) {
        self.middlewares.push(middleware)
    }
}
