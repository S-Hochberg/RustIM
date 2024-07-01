use axum::{debug_handler, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use super::{controllers::{controller::Controller, users_controller::UsersController}, response::ImResponse};
#[derive(Serialize, Deserialize)]
struct TestRes{
	test: String
}

pub fn get_router() -> Router{
	let x = get(test_res);
	let users_ctrl = UsersController::get_ctrl().get_router();
	let r = Router::new()
	.route("/", x)
	.merge(users_ctrl);
	println!("Have router");
	r

}
async fn test_res() -> impl IntoResponse{
	ImResponse{status: StatusCode::ACCEPTED, body: Json(TestRes{test: String::from("Got it")}) }
}