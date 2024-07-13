use crate::{ models::user::user::User, operations::operation::Operation};

pub struct CreateUserState{
	email: String,
	user_name: String
}
pub struct CreateUserOperation{
	state: CreateUserState
}
impl Operation<()> for CreateUserOperation{
	type State = CreateUserState;
	
	async fn execute(&mut self) -> crate::api::response::ImResponse<()> {
		todo!()
	}
	
	fn new(state: CreateUserState) -> Self {
			CreateUserOperation{
				state
			}
		}
}