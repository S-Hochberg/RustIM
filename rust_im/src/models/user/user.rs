use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use display_via_debug::DisplayViaDebug;

#[derive(Deserialize, Serialize, Debug, DisplayViaDebug)]
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