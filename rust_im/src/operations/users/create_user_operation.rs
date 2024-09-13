use std::{fmt::Display};

use anyhow::Error;
use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{ api::response::ImResponse, models::user::user::{User, UserInput}, operations::operation::{OpError, Operation}, repo::{users_repo::users_repo::UsersRepo, DBDrivers}};
#[derive(Serialize, Deserialize)]
pub struct CreateUserOpResponse{
	id: uuid::Uuid
}
pub struct CreateUserOperation{
	state: UserInput
}
impl Operation<CreateUserOpResponse, UserInput> for CreateUserOperation{
	
	async fn execute(&mut self) -> Result<ImResponse<CreateUserOpResponse>, OpError<UserInput>> {
		let user = User::new(self.state.clone());
		let id = user.id.clone();
		let repo = UsersRepo::new(DBDrivers::Postgres);
		repo.create(user).await?;
		Ok(ImResponse{status: StatusCode::CREATED, body: CreateUserOpResponse{ id }})
	}	
	fn new(state: UserInput) -> Self {
			CreateUserOperation{
				state
			}
		}
		
	fn state(&self) -> UserInput {
			self.state.clone()
		}
}