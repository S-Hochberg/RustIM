use macros::DisplayViaDebug;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(TS)]
#[ts(export)]
#[derive(Deserialize, Serialize, Clone, Debug, DisplayViaDebug, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ConversationType {
    Direct,
    Group,
}
#[derive(TS)]
#[ts(export)]
#[derive(Deserialize, Serialize, Clone, Debug, DisplayViaDebug)]
pub struct GroupConversation {
    id: Uuid,
    members: Vec<Uuid>,
    #[serde(with = "utc_date_time")]
    #[ts(as = "created_at")]
    created_at: DateTime<Utc>,
    status: String,
    admin: Uuid,
}
