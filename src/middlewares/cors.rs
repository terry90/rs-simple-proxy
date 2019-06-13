use hyper::header::HeaderValue;
use hyper::{Body, Method, Request, Response};

use crate::proxy::error::MiddlewareError;
use crate::proxy::middleware::MiddlewareResult::Next;
use crate::proxy::middleware::MiddlewareResult::RespondWith;
use crate::proxy::middleware::{Middleware, MiddlewareResult};
use crate::proxy::service::{ServiceContext, State};

pub struct Cors {
    allow_origin: &'static str,
    allow_methods: &'static str,
    allow_headers: &'static str,
}

impl Cors {
    pub fn new(
        allow_origin: &'static str,
        allow_methods: &'static str,
        allow_headers: &'static str,
    ) -> Self {
        Cors {
            allow_origin,
            allow_methods,
            allow_headers,
        }
    }

    fn set_cors_headers(&self, response: &mut Response<Body>) {
        response.headers_mut().insert(
            "Access-Control-Allow-Origin",
            HeaderValue::from_static(self.allow_origin),
        );
        response.headers_mut().insert(
            "Access-Control-Allow-Methods",
            HeaderValue::from_static(self.allow_methods),
        );
        response.headers_mut().insert(
            "Access-Control-Allow-Headers",
            HeaderValue::from_static(self.allow_headers),
        );
    }
}

impl Middleware for Cors {
    fn name() -> String {
        String::from("Cors")
    }

    fn before_request(
        &mut self,
        req: &mut Request<Body>,
        _context: &ServiceContext,
        _state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        if req.method() == Method::OPTIONS {
            let mut response: Response<Body> = Response::new(Body::from(""));
            self.set_cors_headers(&mut response);

            return Ok(RespondWith(response));
        }
        Ok(Next)
    }

    fn after_request(
        &mut self,
        response: Option<&mut Response<Body>>,
        _context: &ServiceContext,
        _state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        if let Some(res) = response {
            self.set_cors_headers(res);
        }
        Ok(Next)
    }
}
