use std::sync::Arc;

use anyhow::{Result};
use axum::{extract::ws::{CloseFrame, Message, WebSocket}, http::StatusCode};
use chrono::{DateTime, Utc};
use dashmap::{mapref::one::RefMut, DashMap};
use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
use tracing::{info, error};
use uuid::Uuid;

use crate::{models::user::{self, user::User}, operation::operation::{OpError, OpErrorInput, Operation, OperationsExecutor}, ws_server::{messaging_service::send_message_to_user, operations::send_mesage_operation::SendMessageOperation}, CONNECTION_MANAGER};

use super::im_message::MessageProtocol;
pub struct ConnectionManager{
	pub connection_map: Arc<DashMap<Uuid, ClientConnection>>
}
impl ConnectionManager{
	pub fn new() -> ConnectionManager{
		ConnectionManager{connection_map: Arc::new(DashMap::new())}
	}
	pub fn insert_connection(&self, conn: ClientConnection) -> (){
		self.connection_map.insert(conn.user.id, conn);
	}
	pub fn remove_connection(&self, user_id: Uuid) -> (){
		self.connection_map.remove(&user_id);
	}
	pub fn connection_exists(&self, user_id: &Uuid) -> bool{
		self.connection_map.contains_key(user_id)
	}
	pub fn get_connection(&self, user_id: &Uuid) -> Option<RefMut<'_, Uuid, ClientConnection>>{
		self.connection_map.get_mut(user_id)
	}
}
 
pub struct ClientConnection{
	pub send_channel: SplitSink<WebSocket, Message>,
	pub connection_time: DateTime<Utc>,
	pub user: User
}
pub async fn create_user_connection(socket: WebSocket, user: User) -> () {
	let (sender, receiver) = socket.split();
	let user_id = user.id.clone();
	let connection = ClientConnection{
		send_channel: sender,
		connection_time: chrono::Utc::now(), 
		user 
	};
	CONNECTION_MANAGER.insert_connection(connection);
	tokio::spawn(read_from_socket(receiver, user_id));
}

async fn read_from_socket(mut receiver: SplitStream<WebSocket>, user_id: Uuid) {
	while let Some(msg) = receiver.next().await {
		if let Ok(msg) = msg {
			match msg {
				Message::Close(close_frame) => {
					handle_disconnect(close_frame, user_id);
					break;
				},
				Message::Text(message) => {
					match serde_json::from_str(message.as_str()){
							Ok(msg) => {
								let op = SendMessageOperation::new(msg);
								OperationsExecutor::execute_op(op).await.unwrap();
							},
							Err(err) => {
								let error_message = MessageProtocol::Error(OpError::bad_request(OpErrorInput{
									message: Some(format!("Failed parsing request message, failed with error {}", err)),
									status: None,
									state: None,
        						}));
								if let Err(err) = send_message_to_user(&error_message, user_id).await{
									match err.status{
										StatusCode::INTERNAL_SERVER_ERROR => {
											error!("Failed sending server error - {error_message} to user {user_id}")
										},
										_ => send_message_to_user(message, target),
									}
								};
							},
						};
				},
				_ => todo!()
			};
		} else {
			handle_disconnect(None, user_id);
			break;
		};
	}
	info!("Disconnected user {}", user_id);
}
fn handle_disconnect(close_frame: Option<CloseFrame>, user_id: Uuid) -> (){
	info!("{:?}", close_frame);
	CONNECTION_MANAGER.remove_connection(user_id);
}

async fn handle_message(message: Message) -> (){
	
}