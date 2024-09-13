use axum::async_trait;
use anyhow::Result;
use crate::{models::user::user::User, operations::{operation::DefaultState, users::user_errors::UserErrors}, CONFIG, IO};

use super::UsersDb;

pub struct PostgresUsersDB();
#[async_trait]
impl UsersDb for PostgresUsersDB{
	async fn create(&self, input: User) -> Result<()>{
		let insert_query = &format!("INSERT INTO {} (id, user_name, email) VALUES ('{}', '{}', '{}')", CONFIG.db.postgres.users_table, input.id, input.user_name, input.email);
		println!("{:?}", insert_query);
		match sqlx::query(insert_query).execute(&IO.get().unwrap().sql).await{
			Ok(val) => Ok(val),
			Err(dbError) => {
				match dbError {
					sqlx::Error::Database(e) if e.message().contains("duplicate key") =>{
						Err(UserErrors::DuplicateUser((), input))
					},
					_ => Err(e)
				}?
			},
		}?;
		let res = sqlx::query(insert_query).execute(&IO.get().unwrap().sql).await?;
		println!("{:?}", res);
		Ok(())
	}
}