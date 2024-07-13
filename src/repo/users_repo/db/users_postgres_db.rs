use sqlx::Executor;

use crate::{config::config::Config, CONFIG, IO};

use super::UsersDb;

pub struct PostgresUsersDB();
impl UsersDb for PostgresUsersDB{
	// async fn create(&self, input: crate::models::user::user::User) -> () {
	// 	let insertQuery = (&format!("
	// 	INSERT INTO {} (id, user_name, email) 
	// 	VALUES ({}, {}, {})", CONFIG.db.postgres.users_table, input.id, input.user_name, input.email));
	// 	sqlx::query(insertQuery).execute(&IO.get().unwrap().sql).await;
	// }
}