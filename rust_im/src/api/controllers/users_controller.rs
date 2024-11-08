use axum::{extract::Path, http::StatusCode, routing::{get, post}, extract, Json};

use tracing::info;
use uuid::Uuid;

use crate::{api::{controllers::controller::{Controller, InternalController}, response::ImResponse, router::REQUEST_CONTEXT}, models::user::user::{PartialUser, User, UserInput}, operations::{operation::{OpError, Operation, OperationsExecutor}, users::{create_user_operation::{CreateUserOpResponse, CreateUserOperation}, get_user_operation::GetUserOperation}}};



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
async fn get_user(Path(user_id): Path<Uuid>) -> Result<ImResponse<User>, OpError<PartialUser>>{
	let op = GetUserOperation::new(PartialUser{id: Some(user_id), email: None, user_name: None});
	OperationsExecutor::execute_op(op).await

}
async fn create_user(extract::Json(user_input): Json<UserInput>) -> Result<ImResponse<CreateUserOpResponse>, OpError<UserInput>>{
	let op = CreateUserOperation::new(user_input);
	OperationsExecutor::execute_op(op).await
}