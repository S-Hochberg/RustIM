use axum::async_trait;
use anyhow::Result;

use crate::models::user::user::User;



#[async_trait]
pub trait UsersDb{
	async fn create(&self, input: User) -> Result<()>;
}
pub mod users_postgres_db;