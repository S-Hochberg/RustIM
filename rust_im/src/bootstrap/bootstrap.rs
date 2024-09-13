use std::{error::Error};

use sqlx::{Connection};
use tokio::{try_join};
use tracing::{info, Level};

use crate::{io, CONFIG, IO};
pub struct Bootstrap{}
impl Bootstrap{
	pub async fn deploy() -> Result<(), Box<dyn Error>>{
		let span = tracing::span!(
			Level::INFO,
			"bootstrap",
		).entered();
		try_join!(Bootstrap::postgres())?;
		span.exit();
		Ok(())
	}
	async fn postgres()-> Result<(), Box<dyn Error>>{
		let db_name = CONFIG.db.postgres.db_name;
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
		sqlx::query(format!(
		"CREATE TABLE IF NOT EXISTS {}(
			id UUID primary key,
			email varchar(1024) unique,
			user_name varchar(128) unique
		)
		", CONFIG.db.postgres.users_table).as_str())
			.execute(&IO.get().unwrap().sql).await?;
		Ok(())
	}
}