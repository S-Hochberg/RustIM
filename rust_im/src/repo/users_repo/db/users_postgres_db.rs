

use axum::{async_trait, http::StatusCode};


use tracing::{error};
use crate::{io::io::IO, models::user::user::{PartialUser, User}, operation::operation::{OpError, OpErrorInput, OpErrorStatus, OpResult, OpType}, CONFIG};

use super::UsersDb;

pub struct PostgresUsersDB();
#[async_trait]
impl UsersDb for PostgresUsersDB{
	async fn create(&self, input: User) -> OpResult<()>{
		let insert_query = &format!("INSERT INTO {} (id, user_name, email) VALUES ('{}', '{}', '{}')", CONFIG.db.postgres.users_table(), input.id, input.user_name, input.email);
		println!("{:?}", insert_query);
		let res = match sqlx::query(insert_query).execute(&IO.get().sql).await{
			Ok(val) => Ok(val),
			Err(db_error) => {
				match db_error {
					sqlx::Error::Database(e) if e.message().contains("duplicate key") =>
						Err(OpError::bad_request(OpErrorInput{message: Some("Duplicate user".to_string()), status: Some(OpErrorStatus::HTTP(StatusCode::BAD_REQUEST)), state: None, op_type: OpType::HTTP })),
					_ => {
						error!("{:?}", db_error);
						Err(OpError::internal_error(&OpType::HTTP))
					}
				}?
			},
		}?;
		println!("{:?}", res);
		Ok(())
	}

	async fn get(&self, input: PartialUser) -> OpResult<User>{
		let filter = 
			match input{
				PartialUser{id: Some(id), email: _, user_name: _} => Ok(format!("id = '{}'", id)),
				PartialUser{email: Some(email), id: _, user_name: _} => Ok(format!("email = '{}'", email)),
				PartialUser{user_name: Some(user_name), email: _, id: _} => Ok(format!("email = '{}'", user_name)),
				PartialUser{id: None, email: None, user_name: None} => Err(OpError{
					message: "Must pass in user id, email, or user_name to get it".to_string(), 
					status: OpErrorStatus::HTTP(StatusCode::BAD_REQUEST),
					state: None
				})
			}?;
		let query = format!(
			"SELECT	id, user_name, email from {}
			Where {}
		", CONFIG.db.postgres.users_table(), filter);
		let res: User = match sqlx::query_as(query.as_str()).fetch_one(&IO.get().sql).await{
			Ok(val) => Ok(val),
			Err(db_error) => {
				match db_error {
					sqlx::Error::RowNotFound =>
						Err(OpError{message: "User not found".to_string(), status: OpErrorStatus::HTTP(StatusCode::NOT_FOUND), state: None }),
					_ => {
						error!("{:?}", db_error);
						Err(OpError::internal_error(&OpType::HTTP))
					}
				}?
			},
		}?;
		Ok(res)
	}
}