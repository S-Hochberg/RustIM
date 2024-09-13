use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct User{
	pub id: Uuid,
	pub email: String,
	pub user_name: String,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserInput{
	pub email: String,
	pub user_name: String
}
impl User{
	pub fn new(input: UserInput) -> User{
		User{id: Uuid::now_v7(), email: input.email, user_name: input.user_name }
	}
}
impl Display for User{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self::Debug{}
	}
}