use std::{borrow::Cow, sync::Arc};

use axum::extract::ws::{CloseFrame, Message, WebSocket};
use chrono::{naive::serde::ts_microseconds::deserialize, DateTime, Utc};
use dashmap::DashMap;
use futures_util::{stream::{SplitSink, SplitStream}, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tracing::{error, info};
use uuid::Uuid;

use crate::{models::user::user::User, ws_server::im_message::{ImMessage, MessageRequest}, CONNECTION_MANAGER};

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
	pub fn remove_connection(&self, user_id: Uuid) -> Option<(Uuid, ClientConnection)>{
		self.connection_map.remove(&user_id)
	}
	pub fn connection_exists(&self, user_id: &Uuid) -> bool{
		self.connection_map.contains_key(user_id)
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
    tokio::spawn(read_from_socket(receiver, user_id));
	let connection = ClientConnection{
		send_channel: sender,
		 connection_time: chrono::Utc::now(), 
		 user 
		};
	CONNECTION_MANAGER.insert_connection(connection)
}

async fn read_from_socket(mut receiver: SplitStream<WebSocket>, user_id: Uuid) {
	let mut frame: Option<CloseFrame> = None;
	while let Some(msg) = receiver.next().await {
		if let Ok(msg) = msg {
			match msg {
				Message::Close(close_frame) => {
					frame = close_frame;
					break;
				},
				Message::Text(value) => {
					match from_str::<MessageRequest>(&value){
						Ok(message_request) => {},
						Err(error) => {
							frame = Some(CloseFrame{code: 1002, reason: Cow::Owned(error.to_string())});
							break;
						}
					}
				},
				_ => {
					frame = Some(CloseFrame{code: 1003, reason: Cow::Borrowed("Only message type text is supported")});
					break;
				}
			};
		} else {
			frame = Some(CloseFrame{code: 1011, reason: Cow::Borrowed("Unable to recieve message")});
			break;
		};
	}
	if let Some((_, value)) = handle_disconnect(&frame, user_id){
		match receiver.reunite(value.send_channel){
			Ok(connection) => {
				if let Err(err) = connection.close().await{
					error!("Failed closing tcp connection for user {} with error {}", user_id, err.to_string());	
				};
			},
			Err(err) => {
				error!("Failed reuniting reader and writer connections for user {} with error {}", user_id, err.to_string());
			},
		};
	};
	info!("Disconnected user {}", user_id);
}
fn handle_disconnect(close_frame: &Option<CloseFrame>, user_id: Uuid) -> Option<(Uuid, ClientConnection)>{
	info!("{:?}", close_frame);
	CONNECTION_MANAGER.remove_connection(user_id)
}