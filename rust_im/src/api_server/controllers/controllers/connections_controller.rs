use std::{error::Error, fmt::Debug};

use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Path}, http::Response, response::IntoResponse, routing::{any, get, post}
};
use uuid::Uuid;


use crate::{models::user::user::{PartialUser, User}, operation::operation::{OpError, OperationsExecutor}, operations::users::get_user_operation::GetUserOperation};
use crate::operation::operation::Operation;

use super::controller::{self, Controller, InternalController};
pub struct ConnectionsController{
	controller: InternalController
}
impl ConnectionsController{
	pub fn get_ctrl() -> Self{
		ConnectionsController::new("/connections")
	}
}
impl Controller for ConnectionsController{
	fn new(base_path: &'static str) -> Self {
		ConnectionsController{controller: InternalController::new(&base_path)}
	}

	fn get_router(self) -> axum::Router {
		self.controller
			.route("/:user_id", get(web_socket_handler))
			.get_router()
	}

}
async fn web_socket_handler(Path(user_id): Path<Uuid>, ws: WebSocketUpgrade) -> Result<impl IntoResponse, OpError<PartialUser>> {
	let op = GetUserOperation::new(PartialUser{id: Some(user_id), email: None, user_name: None});
	let user = OperationsExecutor::execute_op(op).await?;
    Ok(ws.on_upgrade(move |socket| create_user_connection(socket)))
}

async fn create_user_connection(mut socket: WebSocket) {
	println!("ttt");
	while let Some(msg) = socket.recv().await {
		let msg = if let Ok(msg) = msg {
			println!("{:?}", msg);
			msg
		} else {
			// client disconnected
			return;
		};

		if socket.send(msg).await.is_err() {
			// client disconnected
			return;
		}
	}
}