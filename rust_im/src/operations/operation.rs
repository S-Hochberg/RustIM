use std::{ fmt::{Debug, Display}};

use axum::{http::StatusCode, response::IntoResponse};
use anyhow::{Result, Error};
use serde::Serialize;
use tracing::info;

use crate::api::response::ImResponse;

#[derive(Debug)]
pub struct DefaultState{}
impl Display for DefaultState{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Empty state{{}}")
	}
}
#[derive(Debug)]
pub struct OpError<State = DefaultState>
where State: Display + Debug
{
	pub status: StatusCode,
	pub message: String,
	pub state: State
}
impl<State: Display + Debug> Display for OpError<State>{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"Error - message - {}, status - {}, state - {}", self.message, self.status, self.state)
	}
}
impl std::error::Error for OpError{}

impl<State: Display + Debug> IntoResponse for OpError<State>{
	fn into_response(self) -> axum::response::Response {
		(self.status, axum::Json(self.message)).into_response()
	}
}

pub struct OpErrorInput{
	message: String,
	status: Option<StatusCode>
}
impl OpError{
	// pub fn new()
	pub fn message_to_status(message: &String) -> StatusCode{
		match message.to_lowercase(){
			message if message.contains("not found") => StatusCode::NOT_FOUND,
			_ => StatusCode::INTERNAL_SERVER_ERROR
		}
	}
}
pub trait Operation<Res: Serialize, State = DefaultState>
where State: Display + Debug
{
	fn new(s: State) -> Self;
	async fn init(&mut self) -> (){}
	fn state(&self) -> State;
	async fn validate(&mut self) -> (){}
	async fn authorize(&mut self) -> (){}
	fn name(&mut self) -> String{
		std::any::type_name::<Self>().to_string()
	}
	async fn execute(&mut self) -> Result<ImResponse<Res>>;
	async fn on_error(&mut self, err: Error) -> OpError<State>{
		self._default_on_error(err).await
	}
	async fn _default_on_error(&mut self, err: Error) -> OpError<State>{
		let message = err.to_string();
		OpError{
			 status: OpError::message_to_status(&message),
			 message,
			 state: self.state()
		}
	}
}
pub struct OperationsExecutor{}
impl OperationsExecutor{
	pub async fn execute_op<T: Serialize, State: Display + Debug>(mut op: impl Operation<T, State>) -> std::result::Result<ImResponse<T>, OpError<State>>{
		op.init().await;
		op.init().await;
		op.init().await;
		match op.execute().await{
			Ok(res) => Ok(res),
			Err(err) => {
				let err = op.on_error(err).await;
				info!("Error while executing operation {}, error - {:?}",op.name(), err);
				Err(err)

			},
		}
	}
}