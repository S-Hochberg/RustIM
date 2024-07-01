use axum::{ http::{StatusCode}, response::{IntoResponse, Response}};

pub struct ImResponse<R: IntoResponse>{
	pub status: StatusCode,
	pub body: R
}
impl<R: IntoResponse> IntoResponse for ImResponse<R>{
	fn into_response(self) -> Response {
		(self.status, self.body).into_response()
	}
}
