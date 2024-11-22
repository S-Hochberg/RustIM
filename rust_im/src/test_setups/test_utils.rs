#[cfg(test)]
pub mod test_utils{
    use rand::random;
    use uuid::Uuid;

    use crate::{operations::users::create_user_operation::CreateUserOperation, models::user::user::UserInput, operation::{operation::{Operation, OperationsExecutor}}};

	pub fn sample_user_input() -> UserInput{
		let random = random::<u64>().to_string();
		UserInput{
			email: format!("test_{}@rust_im.com", random),
			user_name: format!("test_user_{}", random)
		}
	}

	pub async fn create_test_user() -> Uuid{
		let input = sample_user_input();
		OperationsExecutor::execute_op(CreateUserOperation::new(input)).await.expect("Failed creating a test user").body.id
	}
}