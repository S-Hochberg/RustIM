use axum::async_trait;

use crate::models::user::user::User;

#[async_trait]
pub trait UsersDb{
	// async fn create(&self, input: User) -> ();
}
pub mod users_postgres_db;