use chrono::{DateTime, Utc};
use macros::DisplayViaDebug;
use serde::{Deserialize, Serialize, Serializer};
use ts_rs::TS;
use uuid::Uuid;

use super::conversation::ConversationType;

#[derive(TS)]
#[ts(export)]
#[derive(Deserialize, Serialize, Clone, Debug, DisplayViaDebug)]
pub enum MessageType{
	Text
}
#[derive(TS)]
#[ts(export)]
#[derive(Deserialize, Serialize, Clone, Debug, DisplayViaDebug)]
pub enum ChatOperation{
	Message,
	// Typing,
	// StoppedTyping,
	// Delete,
	// Edit
}

#[derive(TS)]
#[ts(export)]
#[derive(Deserialize, Serialize, Clone, Debug, DisplayViaDebug)]
#[serde(rename_all = "camelCase")]
pub struct MessageRequest{
	pub operation: ChatOperation,
	pub recipient: Uuid,
	pub conversation_type: ConversationType,
	pub message_type: MessageType,
	pub contents: String
}
#[derive(TS)]
#[ts(export)]
#[derive(Deserialize, Serialize, Clone, Debug, DisplayViaDebug)]
#[serde(rename_all = "camelCase")]
pub struct ImMessage{
	pub message_type: MessageType,
	pub content: String,
	pub sender: Uuid,
	pub conversation: Uuid,
	pub conversation_type: ConversationType,
	pub sent_at: DateTime<Utc>,
	pub recieved_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>
}