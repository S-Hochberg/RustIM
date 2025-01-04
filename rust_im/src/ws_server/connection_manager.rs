use std::sync::Arc;

use anyhow::Result;
use axum::extract::ws::{CloseFrame, Message, WebSocket};
use chrono::{DateTime, Utc};
use dashmap::{mapref::one::RefMut, DashMap};
use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
use tracing::info;
use uuid::Uuid;

use crate::{models::user::{self, user::User}, operation::operation::{OpError, OpErrorInput}, CONNECTION_MANAGER};

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
				Message::Text(message) => todo!(),
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