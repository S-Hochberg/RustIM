use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use macros::DisplayViaDebug;

use crate::operation::operation::OpError;
use crate::utils::utils::utc_date_time;
use super::conversation::ConversationType;

#[serde(rename_all="lowercase")]
#[derive(Serialize, Deserialize, Debug, DisplayViaDebug, Clone)]
pub enum MessageType{
	Text
}
#[serde(rename_all="lowercase")]
#[derive(Serialize, Deserialize, Debug, DisplayViaDebug, Clone)]
pub enum ChatOperation{
	Message,
	// Typing,
	// StoppedTyping,
	// Delete,
	// Edit
}
#[serde(rename_all="lowercase")]
#[derive(Serialize, Deserialize, Debug, DisplayViaDebug, Clone)]
pub enum MessageStatus{
	Sent,
	Pending,
	Recieved,
	Read
}

#[derive(Serialize, Deserialize, Debug, DisplayViaDebug, Clone)]
#[serde(tag="protocol", rename_all="lowercase")]
pub enum MessageProtocol{
	Chat(ChatMessage),
	Success(SuccessMessage),
	Failure,
	Error(OpError)

}

#[derive(Serialize, Deserialize, Debug, DisplayViaDebug, Clone)]
#[serde(rename_all="camelCase")]
pub struct ChatMessage{
	pub client_request_id: Uuid,
	pub operation: ChatOperation,
	pub target: Uuid,
	pub conversation_type: ConversationType,
	pub message_type: MessageType,
	pub contents: String
}

#[derive(Serialize, Deserialize, Debug, DisplayViaDebug, Clone)]
#[serde(rename_all="camelCase")]
pub struct SuccessMessage{
	pub client_request_id: Uuid,
	pub message_status: MessageStatus
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