use std::sync::Arc;

use axum::{async_trait, extract::ws::{CloseFrame, Message, WebSocket}};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use futures_util::{stream::{SplitSink, SplitStream}, StreamExt};
use tracing::info;
use uuid::Uuid;

use crate::{models::user::user::User, operation::operation::{OpError, OpErrorInput}, CONNECTION_MANAGER};

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
	while let Some(msg) = receiver.next().await {
		if let Ok(msg) = msg {
			match msg {
				Message::Close(close_frame) => {
					handle_disconnect(close_frame, user_id);
					break;
				},
				_ => {println!("{:?}", msg)}
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