use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{ api::response::ImResponse, models::user::user::{PartialUser, User, UserInput}, operations::operation::{OpError, OpResult, Operation}, repo::{users_repo::users_repo::UsersRepo, DBDrivers}};
#[derive(Serialize, Deserialize)]
pub struct GetUserOperation{
	state: PartialUser
}

impl Operation<User, PartialUser> for GetUserOperation{
	
	async fn execute(&mut self) -> OpResult<ImResponse<User>> {
		let repo = UsersRepo::new(DBDrivers::Postgres);
		let user = repo.get(self.state()).await?;
		Ok(ImResponse{body: user, status: StatusCode::OK})
	}

	fn new(state: PartialUser) -> Self {
			GetUserOperation{
				state
			}
		}

	fn state(&self) -> PartialUser {
			self.state.clone()
		}
}

#[cfg(test)]
mod tests{
    use crate::{models::user::user::{PartialUser, UserInput}, operations::{operation::{Operation, OperationsExecutor}, users::get_user_operation::GetUserOperation}, repo::{users_repo::users_repo::UsersRepo, DBDrivers}, test_setups::{test_setup, test_utils::test_utils::{create_test_user, sample_user_input}}};
	use axum::http::StatusCode;
	use chrono::Utc;
	use lazy_static::lazy_static;
	use rand::random;
	use tracing_test::traced_test;


	#[tokio::test]
	#[traced_test]
	async fn successfully_get_user() -> (){
		let _ctx = test_setup::test_setup::setup().await;
		let user_id = create_test_user().await;
		let op = GetUserOperation::new(PartialUser{id:Some(user_id), email: None, user_name: None });
		let res = OperationsExecutor::execute_op(op).await.unwrap();
		println!("{:?}", res.body);
		assert_eq!(res.status, StatusCode::OK);
		assert_eq!(res.body.id, user_id);
	}
}