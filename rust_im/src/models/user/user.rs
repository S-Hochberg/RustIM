use serde::{Deserialize, Serialize};
use uuid::Uuid;
use macros::{make_partial, DisplayViaDebug};

#[make_partial]
#[derive(Deserialize, Serialize, Debug, DisplayViaDebug, sqlx::FromRow)]
pub struct User{
	pub id: Uuid,
	pub email: String,
	pub user_name: String,
}
#[derive(Deserialize, Serialize, Clone, Debug, DisplayViaDebug)]
pub struct UserInput{
	pub email: String,
	pub user_name: String
}
impl User{
	pub fn new(input: UserInput) -> User{
		User{id: Uuid::now_v7(), email: input.email, user_name: input.user_name }
	}
}