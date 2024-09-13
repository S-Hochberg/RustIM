use std::fmt::Display;

use axum::{debug_handler, extract::Path, http::StatusCode, response::IntoResponse, routing::{get, post}, extract, Json};

use tracing::info;
use uuid::Uuid;

use crate::{api::{controllers::controller::{Controller, InternalController}, response::ImResponse, router::REQUEST_CONTEXT}, models::user::user::{User, UserInput}, operations::{operation::{OpError, Operation, OperationsExecutor}, users::create_user_operation::{CreateUserOpResponse, CreateUserOperation}}};



pub struct UsersController{
	controller: InternalController
}
impl UsersController{
	pub fn get_ctrl() -> Self{
		UsersController::new("/users")
	}
}
impl Controller for UsersController{
	fn new(base_path: &'static str) -> Self {
		UsersController{controller: InternalController::new(&base_path)}
	}

	fn get_router(self) -> axum::Router {
		self.controller
			.route("/:user_id", get(get_user))
			.route("/", post(create_user))
			.get_router()
	}
}
async fn get_user(Path(_user_id): Path<String>) -> Result<ImResponse<User>, OpError>{
	let ctx = REQUEST_CONTEXT.get();
	info!("{:?}", ctx);
	let user = User{ id: Uuid::now_v7(), email: String::from("email"), user_name: String::from("name") };
	Ok(ImResponse{status: StatusCode::OK, body: user })
}
async fn create_user(extract::Json(user_input): Json<UserInput>) -> Result<ImResponse<CreateUserOpResponse>, OpError<UserInput>>{
	let op = CreateUserOperation::new(user_input);
	OperationsExecutor::execute_op(op).await
}