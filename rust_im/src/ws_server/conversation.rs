use chrono::{DateTime, Utc};
use macros::DisplayViaDebug;
use crate::utils::utils::utc_date_time;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, DisplayViaDebug, Clone)]
#[serde(rename_all="lowercase")]
pub enum ConversationType{
	Direct,
	Group
}
#[derive(Serialize, Deserialize, Debug, DisplayViaDebug)]
pub struct GroupConversation{
	id: Uuid,
	members: Vec<Uuid>,
	#[serde(with = "utc_date_time")]
	created_at: DateTime<Utc>,
	status: String,
	admin: Uuid
}