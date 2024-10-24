use axum::http::StatusCode;
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
	
	async fn execute(&mut self) -> Result<ImResponse<CreateUserOpResponse>, OpError> {
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

#[cfg(test)]
mod tests{
    use crate::{models::user::user::UserInput, operations::{operation::{Operation, OperationsExecutor}, users::create_user_operation::CreateUserOperation}, repo::{users_repo::users_repo::UsersRepo, DBDrivers}, test_setups::test_setup};
	use axum::http::StatusCode;
	use chrono::Utc;
	use lazy_static::lazy_static;
	use rand::random;
use tracing_test::traced_test;

	// Keeping this as a reference of how to use lazy static with structs
	// lazy_static!{
	// 	static ref USER_INPUT: UserInput = UserInput{
	// 		email: String::from("Shachar@rust_im.db"),
	// 		user_name: String::from("test_user")
	// 	};
	// Usage:
	// USER_INPUT.clone()
	// }

	fn sample_user_input() -> UserInput{
		let random = random::<u64>().to_string();
		UserInput{
			email: format!("test_{}@rust_im.com", random),
			user_name: format!("test_user_{}", random)
		}
	}
	#[tokio::test]
	#[traced_test]
	async fn succesfully_create_user() -> (){
		let ctx = test_setup::test_setup::setup().await;
		let op = CreateUserOperation::new(sample_user_input());
		let res = OperationsExecutor::execute_op(op).await;
		assert!(res.is_ok());
		if let Ok(result) = res {
			assert_eq!(result.status, StatusCode::CREATED);
		}
		// Make sure the user was actually created in the database with the correct information
	}
	#[tokio::test]
	#[traced_test]
	async fn create_user_that_already_exists() -> (){
		test_setup::test_setup::setup().await;
		let user_input = sample_user_input();
		let op = CreateUserOperation::new(user_input.clone());
		let first_res = OperationsExecutor::execute_op(op).await;
		assert!(first_res.is_ok());
		let second_op = CreateUserOperation::new(user_input.clone());
		let res = OperationsExecutor::execute_op(second_op).await;
		assert!(res.is_err());
		if let Err(err) = res {
			assert_eq!(err.status, StatusCode::BAD_REQUEST);
			assert!(err.message.contains("Duplicate user"));
			let state = err.state.unwrap();
			assert_eq!(state.email, user_input.email);
			assert_eq!(state.user_name, user_input.user_name);
		}
	}
}