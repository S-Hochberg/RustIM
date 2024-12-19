use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::conversation::ConversationType;

pub enum MessageType{
	Text
}
pub enum ChatOperation{
	Message,
	// Typing,
	// StoppedTyping,
	// Delete,
	// Edit
}
pub struct MessageOperation{
	pub contents: String,
	pub message_type : MessageType,
}

pub struct ImMessageProtocol{
	pub operation: ChatOperation,
	pub conversation: Uuid
}
pub struct APIImMessage{
	pub message_type: MessageType,
	pub content: String,
	pub sender: Uuid,
	pub conversation: Uuid,
	pub conversation_type: ConversationType,
	pub sent_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>
}