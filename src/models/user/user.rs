use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct User{
	pub id: Uuid,
	pub email: String,
	pub user_name: String,
}
#[derive(Clone)]
pub struct UserInput{
	pub email: String,
	pub user_name: String
}
impl User{
	pub fn new(input: UserInput) -> User{
		User{id: Uuid::now_v7(), email: input.email, user_name: input.user_name }
	}
}