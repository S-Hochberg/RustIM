use macros::DisplayViaDebug;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(TS)]
#[ts(export)]
#[derive(Deserialize, Serialize, Clone, Debug, DisplayViaDebug)]
pub enum ConversationType{
	Direct,
	Group
}
#[derive(TS)]
#[ts(export)]
#[derive(Deserialize, Serialize, Clone, Debug, DisplayViaDebug)]
pub struct GroupConversation{
	members: Vec<Uuid>,
	admin: Uuid
}