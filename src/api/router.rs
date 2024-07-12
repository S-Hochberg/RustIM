use axum::{body::Body, http::{Request, StatusCode}, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use uuid::Uuid;
use tracing::{Level};


use super::{controllers::{controller::Controller, users_controller::UsersController}, response::ImResponse};
#[derive(Serialize, Deserialize)]
struct TestRes{
	test: String
}

pub fn get_router() -> Router{
	let x = get(test_res);
	let users_ctrl = UsersController::get_ctrl().get_router();
	Router::new()
		.route("/", x)
		.merge(users_ctrl)
		.layer(TraceLayer::new_for_http()
		.on_request(DefaultOnRequest::new().level(Level::INFO))
		.on_response(DefaultOnResponse::new().level(Level::INFO))
		.make_span_with(|request: &Request<Body>| {
			let request_id = Uuid::new_v4();
			tracing::span!(
				Level::INFO,
				"request",
				method = tracing::field::display(request.method()),
				uri = tracing::field::display(request.uri()),
				version = tracing::field::debug(request.version()),
				request_id = tracing::field::display(request_id)
			)})
		)


}
async fn test_res() -> impl IntoResponse{
	ImResponse{status: StatusCode::OK, body: Json(TestRes{test: String::from("Got it")}) }
}