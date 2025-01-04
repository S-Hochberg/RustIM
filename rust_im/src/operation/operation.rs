use std::{ fmt::{Debug, Display}};

use axum::{http::StatusCode, response::{IntoResponse, Response}};
use anyhow::Result;
use macros::DisplayViaDebug;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Deserialize, Serialize)]
pub struct DefaultState{}
impl Display for DefaultState{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Empty state{{}}")
	}
}
pub struct ErrorResponse{
	message: String
}
#[derive(Error, Debug, DisplayViaDebug, Serialize, Deserialize)]
pub struct OpError<State = DefaultState>
where State: Display + Debug
{
	#[serde(with = "http_serde_ext::status_code")]
	pub status: StatusCode,
	pub message: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub state: Option<State>
}

impl<State: Display + Debug> IntoResponse for OpError<State>{
	fn into_response(self) -> axum::response::Response {
		(self.status, self.message).into_response()
	}
}

pub type OpResult<Res> = Result<Res, OpError>;


pub struct ImResponse<R: Serialize>{
	pub status: StatusCode,
	pub body: R
}
impl<R: Serialize> IntoResponse for ImResponse<R>{
	fn into_response(self) -> Response {
		(self.status, axum::Json(self.body)).into_response()
	}
}
pub struct OpErrorInput<State>{
	pub message: Option<String>,
	pub status: Option<StatusCode>,
	pub state: Option<State>
}
impl<State: Debug + Display> OpError<State>{
	pub fn new(input: OpErrorInput<State>) -> Self{
		let message = input.message.unwrap_or(String::from("Internal Error"));
		let status = match input.status{
			Some(status) => status,
			None => OpError::<State>::message_to_status(&message),
		};
		OpError{
			 status,
			 message,
			 state: input.state
			}
	}
	pub fn message_to_status(message: &String) -> StatusCode{
		match message.to_lowercase(){
			message if message.contains("not found") => StatusCode::NOT_FOUND,
			_ => StatusCode::INTERNAL_SERVER_ERROR
		}
	}
	// pub fn concat_message(&mut self, message: String) -> &Self{
	// 	self.message = format!("{} - {}", self.message, message);
	// 	self
	// }
	pub fn internal_error() -> Self{
		OpError{
			message: "Internal Error".to_string(),
			state: None,
			status: StatusCode::INTERNAL_SERVER_ERROR
		}
	}
	pub fn bad_request(input: OpErrorInput<State>) -> OpError<State>{
		let message = input.message.unwrap_or(String::from("Bad Request"));
		OpError::new(OpErrorInput{
			message: Some(message),
			status: Some(StatusCode::BAD_REQUEST),
			state: input.state
		})
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
	async fn execute(&mut self) -> OpResult<ImResponse<Res>>;
	async fn on_error(&mut self, err: OpError) -> OpError<State>{
		self.default_on_error(err)
	}
	fn default_on_error(&mut self, err: OpError) -> OpError<State>{
		OpError{message: err.message, status: err.status, state: Some(self.state())}
	}
}
pub struct OperationsExecutor{}
impl OperationsExecutor{
	pub async fn execute_op<T: Serialize, State: Display + Debug>(mut op: impl Operation<T, State>) -> std::result::Result<ImResponse<T>, OpError<State>>{
		op.init().await;
		op.validate().await;
		op.authorize().await;
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