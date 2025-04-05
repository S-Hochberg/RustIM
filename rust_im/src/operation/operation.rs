use std::{ convert::Infallible, fmt::{Debug, Display}};

use axum::{extract::ws::CloseCode, http::{response::Parts, StatusCode}, response::{IntoResponse, IntoResponseParts, Response, ResponseParts}};
use anyhow::Result;
use macros::{DisplayViaDebug};
use serde::Serialize;
use thiserror::Error;
use tracing::info;

#[derive(Debug)]
pub struct DefaultState{}
impl Display for DefaultState{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Empty state{{}}")
	}
}
pub struct ErrorResponse{
	message: String
}
#[derive(Error, Debug, DisplayViaDebug)]
pub struct OpError<State = DefaultState>
where State: Display + Debug
{
	pub status: OpErrorStatus,
	pub message: String,
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
#[derive(Error, Debug, DisplayViaDebug)]
pub enum OpType{
	HTTP,
	WS
}
#[derive(Error, Debug, DisplayViaDebug, PartialEq, Eq)]
pub enum OpErrorStatus{
	HTTP(StatusCode),
	WS(CloseCode)
}
impl IntoResponseParts for OpErrorStatus {
    type Error = StatusCode;

    fn into_response_parts(self, mut parts: ResponseParts) -> Result<ResponseParts, Self::Error> {
        match self {
            OpErrorStatus::HTTP(status) => {
                Err(status)
            }
            OpErrorStatus::WS(_close_code) => {
                // Should never happen
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

pub struct OpErrorInput<State>{
	pub message: Option<String>,
	pub status: Option<OpErrorStatus>,
	pub state: Option<State>,
	pub op_type: OpType
}
impl<State: Debug + Display> OpError<State>{
	pub fn new(input: OpErrorInput<State>) -> Self{
		let message = input.message.unwrap_or(String::from("Internal Error"));
		let status = match input.status{
			Some(status) => status,
			None => OpError::<State>::message_to_status(&message, &input.op_type)
		};
		OpError{
			 status,
			 message,
			 state: input.state
			}
	}
	pub fn message_to_status(message: &String, op_type: &OpType) -> OpErrorStatus{
		match op_type{
			OpType::WS => match message.to_lowercase(){
				message if message.contains("not found") => OpErrorStatus::WS(1002),
				_ => OpErrorStatus::WS(1011)
			},
			OpType::HTTP => match message.to_lowercase(){
				message if message.contains("not found") => OpErrorStatus::HTTP(StatusCode::NOT_FOUND),
				_ => OpErrorStatus::HTTP(StatusCode::INTERNAL_SERVER_ERROR)
			},
		}
	}
	// pub fn concat_message(&mut self, message: String) -> &Self{
	// 	self.message = format!("{} - {}", self.message, message);
	// 	self
	// }
	pub fn internal_error(op_type: &OpType) -> Self{
		match op_type{
			OpType::HTTP => OpError{
				message: "Internal Error".to_string(),
				state: None,
				status: OpErrorStatus::HTTP(StatusCode::INTERNAL_SERVER_ERROR)
			},
			OpType::WS => OpError{
				message: "Internal Error".to_string(),
				state: None,
				status: OpErrorStatus::WS(1011)
			},
		}
	}
	pub fn bad_request(input: OpErrorInput<State>) -> OpError<State>{
		let message = input.message.unwrap_or(String::from("Bad Request"));
		match input.op_type {
			OpType::HTTP => OpError::new(OpErrorInput{
				message: Some(message),
				status: Some(OpErrorStatus::HTTP(StatusCode::BAD_REQUEST)),
				state: input.state,
				op_type: input.op_type
			}),
			OpType::WS => OpError::new(OpErrorInput{
				message: Some(message),
				status: Some(OpErrorStatus::WS(1002)),
				state: input.state,
				op_type: input.op_type
			}),
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
