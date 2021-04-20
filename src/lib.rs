#[macro_use]
extern crate log;
#[cfg(feature = "router")]
#[macro_use]
extern crate serde_derive;

pub mod middlewares;
pub mod proxy;

use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::Server;
use std::fmt;
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};

use crate::proxy::middleware::Middleware;
use crate::proxy::service::ProxyService;

type Middlewares = Arc<Mutex<Vec<Box<dyn Middleware + Send + Sync>>>>;

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
            _ => Err(String::from(
                "valid values: production, staging, development",
            )),
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

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = ([0, 0, 0, 0], self.port).into();

        info!("Running proxy in {} mode on: {}", self.environment, &addr);

        let middlewares = Arc::clone(&self.middlewares);
        let make_svc = make_service_fn(move |socket: &AddrStream| {
            let remote_addr = socket.remote_addr();
            let middlewares = middlewares.clone();
            debug!("Handling connection for IP: {}", &remote_addr);

            async move { Ok::<_, Infallible>(ProxyService::new(middlewares, remote_addr)) }
        });

        let server = Server::bind(&addr).serve(make_svc);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
        Ok(())
    }

    pub fn add_middleware(&mut self, middleware: Box<dyn Middleware + Send + Sync>) {
        self.middlewares.lock().unwrap().push(middleware)
    }
}
