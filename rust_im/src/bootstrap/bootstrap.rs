use std::{env, error::Error};

use sqlx::{Connection};
use tokio::{try_join};
use tracing::{info, warn, Level};

use crate::{io::{self, io::IO}, test_setups::test_setup::test_setup, CONFIG};
pub enum BootstrapMode{
	Prod,
	Test
}
pub struct PostgresResult{
	pub db_name: String
}
pub struct BootstrapResult{
	pub pg_result: PostgresResult
}
pub struct Bootstrap{}
impl Bootstrap{
	pub async fn deploy(mode: BootstrapMode ) -> Result<BootstrapResult, Box<dyn Error>>{
		let span = tracing::span!(
			Level::INFO,
			"bootstrap",
		).entered();
		let boostrap_results = try_join!(Bootstrap::postgres(mode))?;
		span.exit();
		Ok(BootstrapResult{pg_result: boostrap_results.0})
	}
	async fn postgres(mode: BootstrapMode)-> Result<PostgresResult, Box<dyn Error>>{
		let db_name = match mode{
			BootstrapMode::Prod => CONFIG.db.postgres.db_name().to_string(),
			BootstrapMode::Test => {
				let test_db_name = test_setup::get_test_db_name();
				env::set_var("DB_NAME", &test_db_name);
				test_db_name
			},
		};
		let mut admin = io::db::postgres::postgres::PostrgresConnection::admin().await;
		match sqlx::query(format!("CREATE DATABASE {}", db_name).as_str()
			).execute(&mut admin).await{
			Ok(_) => {},
			Err(err) => {
				match err{
					sqlx::Error::Database(err) => {
						match err.code() {
							Some(val) =>{
								if val != "42P04"{									
									Err(err)?
								}
								info!("Database {} already exists", db_name);
								Ok(())
							},
							None => Err(err)
						}?;
						Ok(())
					},
					_ => Err(err),
				}?
			},
		};

		admin.close().await?;
		let pg_connection = io::db::postgres::postgres::PostrgresConnection::pool().await;
		sqlx::query(format!(
		"CREATE TABLE IF NOT EXISTS {}(
			id UUID primary key,
			email varchar(1024) unique,
			user_name varchar(128) unique
		)
		", CONFIG.db.postgres.users_table()).as_str())
			.execute(&pg_connection).await?;
		pg_connection.close().await;
		Ok(PostgresResult{db_name})
	}
}