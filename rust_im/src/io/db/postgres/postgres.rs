use sqlx::{postgres::PgPoolOptions, Connection, PgConnection, PgPool};

use crate::CONFIG;

pub struct PostrgresConnection{}
impl PostrgresConnection{
	pub async fn admin() -> PgConnection{
		PgConnection::connect(CONFIG.db.postgres.db_url().as_str()).await.expect("Failed initializing admin postgres connection")
	}
	pub async fn pool() -> PgPool{
		PgPoolOptions::new()
        	.max_connections(10)
        	.connect(&format!("{}/{}", CONFIG.db.postgres.db_url(), CONFIG.db.postgres.db_name())).await.expect("Failed initializing postgres pool connection")
	}
	pub async fn _connection() -> PgConnection{
		PgConnection::connect(&format!("{}/{}", CONFIG.db.postgres.db_url(), CONFIG.db.postgres.db_name())).await.expect("Failed initializing postgres connection")
	}
	
}