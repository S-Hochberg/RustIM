

use axum::async_trait;
use anyhow::Result;

use crate::{models::user::user::{PartialUser, User}, operations::operation::OpError};



#[async_trait]
pub trait UsersDb{
	async fn create(&self, input: User) -> Result<(), OpError>;
	async fn get(&self, input: PartialUser) -> Result<User, OpError>;
}
pub mod users_postgres_db;