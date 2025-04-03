use serde::{Deserialize, Serialize};
use uuid::Uuid;
use macros::{make_partial, DisplayViaDebug};
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
#[make_partial]
#[derive(Deserialize, Serialize, Debug, DisplayViaDebug, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User{
	pub id: Uuid,
	pub email: String,
	pub user_name: String,
}
#[derive(TS)]
#[ts(export)]
#[derive(Deserialize, Serialize, Clone, Debug, DisplayViaDebug)]
#[serde(rename_all = "camelCase")]
pub struct UserInput{
	pub email: String,
	pub user_name: String
}
impl User{
	pub fn new(input: UserInput) -> User{
		User{id: Uuid::now_v7(), email: input.email, user_name: input.user_name }
	}
}