

use axum::async_trait;
use anyhow::Result;

use crate::{models::user::user::User, operations::operation::OpError};



#[async_trait]
pub trait UsersDb{
	async fn create(&self, input: User) -> Result<(), OpError>;
}
pub mod users_postgres_db;