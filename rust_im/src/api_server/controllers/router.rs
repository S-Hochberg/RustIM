use std::{fmt, time::Duration};

use axum::{body::Body, http::{Method, Request, Response, Version}, middleware::{self, Next}, Router};
use serde::{Deserialize, Serialize};
use tokio::task_local;
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use tracing::{debug, info, span, Level, Span};


use super::controllers::{connections_controller::{ConnectionsController}, controller::Controller, users_controller::UsersController};
struct RequestLatency(Duration);

impl fmt::Display for RequestLatency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ms", self.0.as_millis())
    }
}


#[derive(Clone, Debug)]
pub struct RequestContext{
	pub request_id: Uuid,
	pub method: Method,
	pub uri: String,
	pub version: Version
}

task_local! {
    pub static REQUEST_CONTEXT: RequestContext
}
async fn request_info_middleware(req: Request<Body>, next: Next) -> Response<Body> {
	let request_id = Uuid::new_v4();
    let uri = req.uri().to_string();
    let method = req.method().clone();
	let version = req.version();
    let request_info = RequestContext { request_id, method, uri, version };
	REQUEST_CONTEXT.scope(request_info, async move {
		next.run(req).await
	}).await
}

#[derive(Serialize, Deserialize)]
struct TestRes{
	test: String
}

pub fn get_router() -> Router{
	let users_ctrl = UsersController::get_ctrl().get_router();
	let connections_controller = ConnectionsController::get_ctrl().get_router();
	Router::new()
		.merge(users_ctrl)
		.merge(connections_controller)
	.layer(
		TraceLayer::new_for_http()
		.make_span_with(|_request: &Request<Body>| {
			let context = REQUEST_CONTEXT.get();
			tracing::span!(
				Level::INFO,
				"request",
				request_id = tracing::field::display(context.request_id)
			)})
			.on_request(|request: &Request<Body>, _span: &Span| {
				let span = tracing::span!(
					Level::INFO,
					"request",
					method = tracing::field::display(request.method()),
					uri = tracing::field::display(request.uri()),
					version = tracing::field::debug(request.version()),
				);
				info!(parent: &span, "started processing request");
			}
		)
		.on_response(|response: &Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
			let context = REQUEST_CONTEXT.get();
			let status = response.status().to_string();
			println!("{:?}", response);
			println!("{:?}",response.body());
			let latency = RequestLatency(latency);
			let response_span = span!(
				Level::INFO,
				"response",
				method = tracing::field::display(context.method.clone()),
				uri = tracing::field::display(context.uri.clone()),
				version = tracing::field::debug(context.version),
				latency = tracing::field::display(latency)
			);
			response_span.in_scope(|| {
				debug!("Sent response with status: {}", status);
				debug!("Sent response with body: {:?}", response.body());
			});
		})
	)
	.layer(middleware::from_fn(request_info_middleware))
}