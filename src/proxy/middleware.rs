use crate::proxy::error::MiddlewareError;
use crate::proxy::service::{ServiceContext, State};
use hyper::{Body, Error, Request, Response};

pub enum MiddlewareResult {
    RespondWith(Response<hyper::Body>),
    Next,
}

use self::MiddlewareResult::Next;

pub trait Middleware {
    fn name() -> String
    where
        Self: Sized;

    fn get_name(&self) -> String
    where
        Self: Sized,
    {
        Self::name()
    }

    fn set_state(&self, req_id: u64, state: &State, data: String) -> Result<(), MiddlewareError>
    where
        Self: Sized,
    {
        let mut state = state.lock()?;
        state.insert((self.get_name(), req_id), data);
        Ok(())
    }

    fn state(req_id: u64, state: &State) -> Result<Option<String>, MiddlewareError>
    where
        Self: Sized,
    {
        let state = state.lock()?;
        debug!("State length: {}", state.len());
        let state = match state.get(&(Self::name(), req_id)) {
            None => None,
            Some(state) => Some(state.to_string()),
        };

        debug!(
            "[{}] State for {}: {:?}",
            Self::name(),
            &req_id.to_string()[..6],
            state
        );

        Ok(state)
    }

    fn get_state(&self, req_id: u64, state: &State) -> Result<Option<String>, MiddlewareError>
    where
        Self: Sized,
    {
        Self::state(req_id, state)
    }

    fn before_request(
        &mut self,
        _req: &mut Request<Body>,
        _ctx: &ServiceContext,
        _state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        Ok(Next)
    }

    fn after_request(
        &mut self,
        _res: Option<&mut Response<Body>>,
        _ctx: &ServiceContext,
        _state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        Ok(Next)
    }

    fn request_failure(
        &mut self,
        _err: &Error,
        _ctx: &ServiceContext,
        _state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        Ok(Next)
    }

    fn request_success(
        &mut self,
        _res: &mut Response<Body>,
        _ctx: &ServiceContext,
        _state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        Ok(Next)
    }
}
