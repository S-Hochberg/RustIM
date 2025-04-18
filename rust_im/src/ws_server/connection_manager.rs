use std::{borrow::Cow, sync::Arc};

use axum::{extract::ws::{CloseFrame, Message, WebSocket}, http::StatusCode};
use chrono::{naive::serde::ts_microseconds::deserialize, DateTime, Utc};
use dashmap::DashMap;
use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use tokio::{select, task::JoinHandle};
use tracing::{error, info};
use uuid::Uuid;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{models::user::user::User, operation::operation::{OpError, OpErrorInput, OpErrorStatus, OpType, OperationsExecutor, Operation}, operations::messages::send_message_operation::{SendMessageOperation, SendMessageRequest}, ws_server::im_message::{ImMessage, MessageRequest}, CONNECTION_MANAGER};

use super::conversation::ConversationType;

use super::im_message::MessageProtocol;
pub struct ConnectionManager{
	pub connection_map: Arc<DashMap<Uuid, ClientConnection>>,
	task_tracker: TaskTracker,
	task_cancellation_token: CancellationToken

}

enum DisconnectInitiator{
	Client,
	Server
}
struct ConnectionDisconnect<'a>{
	initiator: DisconnectInitiator,
	frame: Option<CloseFrame<'a>>
}

impl ConnectionManager{
	pub fn new() -> ConnectionManager{
		ConnectionManager{connection_map: Arc::new(DashMap::new()), task_tracker: TaskTracker::new(), task_cancellation_token: CancellationToken::new()}
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
	pub async fn handle_shutdown(&self){
		self.task_tracker.close();
		self.task_cancellation_token.cancel();
		self.task_tracker.wait().await;
	}
	pub async fn send_message_to_user(&self, message: ImMessage) -> Result<(), OpError>{
		match message.conversation_type{
			super::conversation::ConversationType::Direct => {
				match self.connection_map.get_mut(&message.recipient){
						Some(mut user) => {
							match to_string::<ImMessage>(&message){
									Ok(stringified) => {
										user.send_channel.send(Message::Text(stringified)).await.unwrap();
										Ok(())
									},
									Err(err) => {
										error!("Error while stringifying message {:?}, error - {}", message, err);
										Err(OpError::internal_error(&OpType::WS))
									},
								}
						},
						None => {
							// TODO: offline user
							info!("User {} is disconnected", &message.recipient);
							Err(OpError::bad_request(OpErrorInput{message: Some(format!("User {} is offline", &message.recipient)), status: Some(OpErrorStatus::WS(1000)), state: None, op_type: OpType::WS}))
						},
					}
			},
			super::conversation::ConversationType::Group => todo!(),
		}
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
    CONNECTION_MANAGER.task_tracker.spawn(handle_connection(receiver, user_id, CONNECTION_MANAGER.task_cancellation_token.clone()));
	let connection = ClientConnection{
		send_channel: sender,
		 connection_time: chrono::Utc::now(), 
		 user,
		};
	CONNECTION_MANAGER.insert_connection(connection)
}

async fn handle_connection(mut receiver: SplitStream<WebSocket>, user_id: Uuid, cancellation_token: CancellationToken){
	// let read =  read_from_socket(&receiver, user_id);
	let cancelled = cancellation_token.cancelled();
	let disconnect = select! {
		disconnect = async{
			let mut frame: Option<CloseFrame> = None;
			let mut initiator: DisconnectInitiator = DisconnectInitiator::Server;
			while let Some(msg) = receiver.next().await { 
				if let Ok(msg) = msg {
					match msg {
						Message::Close(close_frame) => {
							initiator = DisconnectInitiator::Client;
							frame = close_frame;
							break;
						},
						Message::Text(value) => {
							match from_str::<MessageRequest>(&value){
								Ok(message_request) => {
									// CONNECTION_MANAGER.send_message_to_user(message_request).await;
									let op = SendMessageOperation::new(SendMessageRequest{request: message_request, sender: user_id});
									match OperationsExecutor::execute_op(op).await{
											Ok(response) => {
												// Todo: Implement message received back to sender
											},
											Err(err) => {
												let code = match err.status{
													OpErrorStatus::WS(code) => code,
													_ => 1011
												};
												frame = Some(CloseFrame { code, reason: Cow::from(err.message) });
												break;
											},
										};
								},
								Err(error) => {
									info!("{}",error);
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
			let disconnect = ConnectionDisconnect{frame, initiator};
			return disconnect
		} => {
			disconnect
		}
		_ = cancelled => {ConnectionDisconnect{frame: Some(CloseFrame{code: 1001, reason: Cow::from("Server is shutting down")}), initiator: DisconnectInitiator::Server}}
	};
	info!("Disconnecting user {} - {:?}", user_id, disconnect.frame);
	match disconnect.initiator{
		DisconnectInitiator::Client => {
			CONNECTION_MANAGER.remove_connection(user_id);
			info!("User {} disconnected with frame {:?}", user_id, disconnect.frame)
		},
		DisconnectInitiator::Server => {
			if let Some((_, value)) = CONNECTION_MANAGER.remove_connection(user_id){
				match receiver.reunite(value.send_channel){
					Ok(mut connection) => {
						if let Err(err) = connection.send(Message::Close(disconnect.frame)).await {
							error!("Failed to send custom close frame for user {}: {}", user_id, err);
						}
						if let Err(err) = connection.close().await{
							if !err.to_string().contains("already closed") {
								error!("Failed closing tcp connection for user {} with error {}", user_id, err.to_string());	
							}
						};
					},
					Err(err) => {
						error!("Failed reuniting reader and writer connections for user {} with error {}", user_id, err.to_string());
					},
				};
			};
		},
	}
	info!("Disconnected user {}", user_id);
}
