use chrono::{DateTime, Utc};
use macros::DisplayViaDebug;
use serde::{Deserialize, Serialize, Serializer};
use ts_rs::TS;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use macros::DisplayViaDebug;

use crate::operation::operation::OpError;
use crate::utils::utils::utc_date_time;
use super::conversation::ConversationType;

#[derive(TS)]
#[ts(export)]
#[derive(Deserialize, Serialize, Clone, Debug, DisplayViaDebug, Copy)]
#[serde(rename_all="lowercase")]
#[derive(Serialize, Deserialize, Debug, DisplayViaDebug, Clone)]
pub enum MessageType{
	Text
}

#[serde(rename_all="lowercase")]
#[derive(Serialize, Deserialize, Debug, DisplayViaDebug, Clone)]
pub enum ChatOperation{
	Message,
	MessageReceived
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
pub enum MessageRequest{
	Chat(ChatMessage),
	Success(SuccessMessage),
	Error(OpError)

}

#[derive(TS)]
#[ts(export)]
#[derive(Deserialize, Serialize, Clone, Debug, DisplayViaDebug)]
#[serde(rename_all = "camelCase")]
pub struct MessageRequest{
	pub client_request_id: Uuid,
	pub operation: ChatOperation,
	pub target: Uuid,
	pub conversation_type: ConversationType,
	pub message_type: MessageType,
	pub content: String
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
	pub conversation_type: ConversationType,
	pub recipient: Uuid,
	pub sent_at: DateTime<Utc>,
	pub recieved_at: Option<DateTime<Utc>>,
	#[serde(with = "utc_date_time")]
	pub sent_at: DateTime<Utc>,
	#[serde(with = "utc_date_time")]
	pub updated_at: DateTime<Utc>
}