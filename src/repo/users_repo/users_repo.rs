use sqlx::Postgres;

use crate::repo::DBDrivers;

use super::db::{users_postgres_db::PostgresUsersDB, UsersDb};

use std::fmt::Display;
pub trait Test{}
struct UsersRepo{
	db: Box<dyn UsersDb>
}
impl UsersRepo{
	fn new(db_driver: DBDrivers) -> Self{
		let db = match db_driver{
			DBDrivers::Postgres => PostgresUsersDB{},
		};
		UsersRepo{
			db: Box::new(db)
		}
	}
}