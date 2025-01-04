use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::operation::operation::OpError;
use crate::utils::utils::utc_date_time;
use super::conversation::ConversationType;

#[derive(Serialize, Deserialize)]
pub enum MessageType{
	Text
}
#[derive(Serialize, Deserialize)]
pub enum ChatOperation{
	Message,
	// Typing,
	// StoppedTyping,
	// Delete,
	// Edit
}
#[derive(Serialize, Deserialize)]
#[serde(tag="protocol")]
pub enum MessageProtocol{
	Chat(ChatProtocol),
	// Server(ServerProtocol),
	Error(OpError)
	
}

#[derive(Serialize, Deserialize)]
pub struct ChatProtocol{
	pub operation: ChatOperation,
	pub recipient: Uuid,
	pub conversation_type: ConversationType,
	pub message_type: MessageType,
	pub contents: String
}
#[derive(Serialize, Deserialize)]
pub struct ImMessage{
	pub message_type: MessageType,
	pub content: String,
	pub sender: Uuid,
	pub conversation: Uuid,
	pub conversation_type: ConversationType,
	#[serde(with = "utc_date_time")]
	pub sent_at: DateTime<Utc>,
	#[serde(with = "utc_date_time")]
	pub updated_at: DateTime<Utc>
}