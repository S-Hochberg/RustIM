use sqlx::PgPool;

use super::db::postgres::postgres::PostrgresConnection;

#[derive(Debug)]
pub struct Io{
	pub sql: PgPool
}
impl Io{
	pub async fn init() -> Self{
		Io { 
			sql: PostrgresConnection::pool().await 
		}
	}
}