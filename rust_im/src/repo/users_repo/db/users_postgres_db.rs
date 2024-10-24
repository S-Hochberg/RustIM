

use axum::{async_trait, http::StatusCode};
use anyhow::Result;

use tracing::{error};
use crate::{models::user::user::User, operations::operation::OpError, CONFIG, io::io::IO};

use super::UsersDb;

pub struct PostgresUsersDB();
#[async_trait]
impl UsersDb for PostgresUsersDB{
	async fn create(&self, input: User) -> Result<(), OpError>{
		let insert_query = &format!("INSERT INTO {} (id, user_name, email) VALUES ('{}', '{}', '{}')", CONFIG.db.postgres.users_table(), input.id, input.user_name, input.email);
		println!("{:?}", insert_query);
		let res = match sqlx::query(insert_query).execute(&IO.get().sql).await{
			Ok(val) => Ok(val),
			Err(db_error) => {
				match db_error {
					sqlx::Error::Database(e) if e.message().contains("duplicate key") =>
						Err(OpError{message: "Duplicate user".to_string(), status: StatusCode::BAD_REQUEST, state: None }),
					_ => {
						error!("{:?}", db_error);
						Err(OpError::internal_error())
					}
				}?
			},
		}?;
		println!("{:?}", res);
		Ok(())
	}
}