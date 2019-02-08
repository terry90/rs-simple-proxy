use hyper::{Body, Request, Response};

use crate::proxy::error::MiddlewareError;
use crate::proxy::middleware::MiddlewareResult::{Next, RespondWith};
use crate::proxy::middleware::{Middleware, MiddlewareResult};
use crate::proxy::service::State;

pub struct Health {
    route: &'static str,
    raw_body: &'static str,
}

impl Health {
    pub fn new(route: &'static str, raw_body: &'static str) -> Self {
        Health {
            route: route,
            raw_body: raw_body,
        }
    }
}

impl Middleware for Health {
    fn name() -> String {
        String::from("Health")
    }

    fn before_request(
        &mut self,
        req: &mut Request<Body>,
        _req_id: u64,
        _state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        if req.uri().path() == self.route {
            let ok: Response<Body> = Response::new(Body::from(self.raw_body));
            return Ok(RespondWith(ok));
        }
        Ok(Next)
    }
}
