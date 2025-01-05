use std::{any::Any, sync::Arc};

use anyhow::{Result};
use axum::{extract::ws::{CloseFrame, Message, WebSocket}, http::StatusCode};
use chrono::{DateTime, Utc};
use dashmap::{mapref::one::RefMut, DashMap};
use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
use tracing::{info, error};
use uuid::Uuid;

use crate::{models::user::{self, user::User}, operation::operation::{OpError, OpErrorInput, Operation, OperationsExecutor}, ws_server::{im_message::SuccessMessage, messaging_service::{send_error_to_user, send_message_to_user}, operations::send_mesage_operation::SendMessageOperation}, CONNECTION_MANAGER};

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
								match OperationsExecutor::execute_op(op).await {
											Ok(res) => {
												let success_message = SuccessMessage{
                									client_request_id: res.body.client_message_id,
                									message_status: res.body.message_status
												}; 
												// TODO: handle success message failure (should probably be the same handling as afailing to send an error message)
												send_message_to_user(&MessageProtocol::Success(success_message), user_id).await.unwrap();
											},
											Err(err) => {
												send_error_to_user(err.into_default_state(), user_id).await;
											},
										};
							},
							Err(err) => {
								let op_error: OpError = OpError::bad_request(OpErrorInput{
									message: Some(format!("Failed parsing request message, failed with error {}", err)),
									status: None,
									state: None,
        						});
								send_error_to_user(op_error, user_id).await;
							},
						};
				},
				val => {
					let op_error: OpError = OpError::bad_request(OpErrorInput{
						message: Some(format!("Invalid web socket message type - expected type 'text'")),
						status: None,
						state: None,
					});
					send_error_to_user(op_error, user_id).await;


				}
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