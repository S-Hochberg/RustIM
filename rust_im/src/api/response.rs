use axum::{ http::{StatusCode}, response::{IntoResponse, Response}};
use serde::Serialize;

pub struct ImResponse<R: Serialize>{
	pub status: StatusCode,
	pub body: R
}
impl<R: Serialize> IntoResponse for ImResponse<R>{
	fn into_response(self) -> Response {
		(self.status, axum::Json(self.body)).into_response()
	}
}