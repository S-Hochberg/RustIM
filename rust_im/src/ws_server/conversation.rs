use chrono::{DateTime, Utc};
use crate::utils::utils::utc_date_time;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum ConversationType{
	Direct,
	Group
}
#[derive(Serialize, Deserialize)]
pub struct GroupConversation{
	id: Uuid,
	members: Vec<Uuid>,
	#[serde(with = "utc_date_time")]
	created_at: DateTime<Utc>,
	status: String,
	admin: Uuid
}