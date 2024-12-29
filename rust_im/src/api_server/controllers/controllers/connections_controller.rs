use axum::{
    extract::{ws::WebSocketUpgrade, Path}, response::IntoResponse, routing::get
};
use uuid::Uuid;


use crate::{models::user::user::PartialUser, operation::operation::{OpError, OpErrorInput, OperationsExecutor}, operations::users::get_user_operation::GetUserOperation, ws_server::connection_manager::create_user_connection, CONNECTION_MANAGER};
use crate::operation::operation::Operation;

use super::controller::{Controller, InternalController};
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
			.route("/:user_id", get(new_connection_handler))
			.get_router()
	}
}

async fn new_connection_handler(Path(user_id): Path<Uuid>, ws: WebSocketUpgrade) -> Result<impl IntoResponse, OpError<PartialUser>> {
	let op = GetUserOperation::new(PartialUser{id: Some(user_id), email: None, user_name: None});
	let user = OperationsExecutor::execute_op(op).await?;
	if CONNECTION_MANAGER.connection_exists(&user_id){
		let err = OpError::bad_request(OpErrorInput{
			message: Some(format!("User {} already connected, only one user connection is allowed at a time", user_id)),
			status: None, 
			state: None });
		return Err(err)	
	}
    let res = ws.on_upgrade(move |socket| async {create_user_connection(socket, user.body).await});
	Ok(res)
}