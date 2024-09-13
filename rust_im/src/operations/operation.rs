use std::{ fmt::{Debug, Display}};

use axum::{http::StatusCode, response::IntoResponse};
use anyhow::{Result, Error};
use display_via_debug::DisplayViaDebug;
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
#[derive(Debug, DisplayViaDebug)]
pub struct OpError<State = DefaultState>
where State: Display + Debug
{
	pub status: StatusCode,
	pub message: String,
	pub state: Option<State>
}
impl std::error::Error for OpError{}

impl<State: Display + Debug> IntoResponse for OpError<State>{
	fn into_response(self) -> axum::response::Response {
		(self.status, axum::Json(self.message)).into_response()
	}
}

pub struct OpErrorInput<State>{
	message: String,
	status: Option<StatusCode>,
	state: Option<State>
}
impl<State: Debug + Display> OpError<State>{
	pub fn new(input: OpErrorInput<State>) -> Self{
		let status = match input.status{
			Some(status) => status,
			None => OpError::<State>::message_to_status(&input.message),
		};
		OpError{
			 status,
			 message: input.message,
			 state: input.state
			}
	}
	pub fn message_to_status(message: &String) -> StatusCode{
		match message.to_lowercase(){
			message if message.contains("not found") => StatusCode::NOT_FOUND,
			_ => StatusCode::INTERNAL_SERVER_ERROR
		}
	}
	pub fn concat_message(&mut self, message: String) -> (){
		self.message = format!("{} - {}", self.message, message)
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
	async fn execute(&mut self) -> Result<ImResponse<Res>, OpError<State>>;
	async fn on_error(&mut self, err: OpError<State>) -> OpError<State>{
		err
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