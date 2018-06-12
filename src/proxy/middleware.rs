use hyper::{Body, Error, Request, Response};
use proxy::error::MiddlewareError;
use proxy::service::State;

pub enum MiddlewareResult {
  RespondWith(Response<Body>),
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
    state.insert((self.get_name().to_string(), req_id), data);
    Ok(())
  }

  fn state(req_id: u64, state: &State) -> Result<String, MiddlewareError>
  where
    Self: Sized,
  {
    let state = state.lock()?;
    debug!("State length: {}", state.len());
    let state = state
      .get(&(Self::name(), req_id))
      .expect(&format!("[{}] State is corrupt", Self::name())); // TODO: Gracefuly fail

    debug!(
      "[{}] State for {}: {:?}",
      Self::name(),
      &req_id.to_string()[..6],
      state
    );

    Ok(state.to_string())
  }

  fn get_state(&self, req_id: u64, state: &State) -> Result<String, MiddlewareError>
  where
    Self: Sized,
  {
    Self::state(req_id, state)
  }

  fn before_request(
    &mut self,
    _req: &mut Request<Body>,
    _req_id: u64,
    _state: &State,
  ) -> Result<MiddlewareResult, MiddlewareError> {
    Ok(Next)
  }

  fn after_request(
    &mut self,
    _req_id: u64,
    _state: &State,
  ) -> Result<MiddlewareResult, MiddlewareError> {
    Ok(Next)
  }

  fn request_failure(
    &mut self,
    _err: &Error,
    _req_id: u64,
    _state: &State,
  ) -> Result<MiddlewareResult, MiddlewareError> {
    Ok(Next)
  }

  fn request_success(
    &mut self,
    _req: &mut Response<Body>,
    _req_id: u64,
    _state: &State,
  ) -> Result<MiddlewareResult, MiddlewareError> {
    Ok(Next)
  }
}
