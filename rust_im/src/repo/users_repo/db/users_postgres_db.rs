use axum::{async_trait, http::StatusCode};
use anyhow::Result;
use crate::{models::user::user::{User, UserInput}, operations::{operation::{DefaultState, OpError, OpErrorInput}, users::user_errors::USER_ERRORS}, CONFIG, IO};

use super::UsersDb;

pub struct PostgresUsersDB();
#[async_trait]
impl UsersDb for PostgresUsersDB{
	async fn create(&self, input: User) -> Result<(), OpError<UserInput>>{
		let insert_query = &format!("INSERT INTO {} (id, user_name, email) VALUES ('{}', '{}', '{}')", CONFIG.db.postgres.users_table, input.id, input.user_name, input.email);
		println!("{:?}", insert_query);
		match sqlx::query(insert_query).execute(&IO.get().unwrap().sql).await{
			Ok(val) => Ok(val),
			Err(dbError) => {
				match dbError {
					sqlx::Error::Database(e) if e.message().contains("duplicate key") =>{
						Err(USER_ERRORS.DuplicateUser.concat_message(e.message().to_string()))
					},
					_ => Err(OpError::new(OpErrorInput{ message: dbError.to_string(), status: todo!(), state: todo!() }))
				}?
			},
		}?;
		let res = sqlx::query(insert_query).execute(&IO.get().unwrap().sql).await?;
		println!("{:?}", res);
		Ok(())
	}
}