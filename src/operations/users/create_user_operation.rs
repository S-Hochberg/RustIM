use axum::http::StatusCode;

use crate::{ api::response::ImResponse, models::user::user::{User, UserInput}, operations::operation::Operation};

pub struct CreateUserOperation{
	state: UserInput
}
impl Operation<()> for CreateUserOperation{
	type State = UserInput;
	
	async fn execute(&mut self) -> crate::api::response::ImResponse<()> {
		let user = User::new(self.state.clone());
		ImResponse{status: StatusCode::CREATED, body: ()}
	}	
	fn new(state: UserInput) -> Self {
			CreateUserOperation{
				state
			}
		}
}