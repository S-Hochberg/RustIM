use std::time::Duration;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json};
use tokio::time::sleep;
use tracing::info;
use uuid::Uuid;

use crate::{api::{controllers::controller::{Controller, InternalController}, response::ImResponse, router::REQUEST_CONTEXT}, models::user::user::User};
use crate::models::user::user;


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
			.get_router()
	}
}
async fn get_user(Path(user_id): Path<String>) -> impl IntoResponse{
	let ctx = REQUEST_CONTEXT.get();
	info!("{:?}", ctx);
	ImResponse{status: StatusCode::OK, body: Json(User{ id: Uuid::now_v7(), email: String::from("email"), user_name: String::from("name") }) }

}