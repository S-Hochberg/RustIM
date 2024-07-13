use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct User{
	pub id: Uuid,
	pub email: String,
	pub user_name: String,
}
struct UserInput{
	email: String,
	user_name: String
}
impl User{
	fn new(input: UserInput) -> User{
		User {id: Uuid::now_v7(), ..input}
	}
}