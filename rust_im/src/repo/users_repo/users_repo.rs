use crate::{models::user::user::{User}, operations::operation::{OpError}, repo::DBDrivers};
use anyhow::Result;

use super::db::{users_postgres_db::PostgresUsersDB, UsersDb};


pub struct UsersRepo{
	db: Box<dyn UsersDb + Send + Sync>
}
impl UsersRepo{
	pub fn new(db_driver: DBDrivers) -> Self{
		let db = match db_driver{
			DBDrivers::Postgres => PostgresUsersDB{},
		};
		UsersRepo{
			db: Box::new(db)
		}
	}
	pub async fn create(&self, user: User) -> Result<(), OpError>{
		self.db.create(user).await?;
		Ok(())
	}
}