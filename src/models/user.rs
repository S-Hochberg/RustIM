use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User{
	pub id: String,
	pub email: String,
	pub name: String,
}