use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json};

use crate::{api::{controllers::controller::{Controller, InternalController}, response::ImResponse}};
use crate::models::user::User;


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
	// String::from("email")
	ImResponse{status: StatusCode::ACCEPTED, body: Json(User{ id: user_id, email: String::from("email"), name: String::from("name") }) }

}