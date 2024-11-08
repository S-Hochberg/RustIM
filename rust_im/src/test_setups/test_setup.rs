
// #[cfg(test)]
pub mod test_setup{
    use std::{env, time::SystemTime};
	use tokio::sync::OnceCell;
	use chrono::offset::Utc;
	use sqlx::Row; 
    use crate::{bootstrap, io::{db::postgres, io::Io}, repo::{users_repo::users_repo::UsersRepo, DBDrivers}};
	pub static TEST_SETUP_DONE: OnceCell<()> = OnceCell::const_new();
	pub struct TestContext{
		users_repo: UsersRepo
	}
	const TEST_DB_PREFIX: &str = "test_rust_im_";
	pub fn get_test_db_name() -> String{
		let time = Utc::now().timestamp();
		format!("{}{}", TEST_DB_PREFIX, time)

	}
	async fn teardown_test_dbs() -> (){
		let mut admin = postgres::postgres::PostrgresConnection::admin().await;
		let q = format!("SELECT datname from pg_database where datname like '{}%'", TEST_DB_PREFIX);
		let test_dbs = sqlx::query(q.as_str()
			).fetch_all(&mut admin).await.unwrap();
		for row in test_dbs{
			let db: &str = row.get("datname");
			let drop_query = format!("DROP DATABASE {}", db);
			sqlx::query(&drop_query).execute(&mut admin).await.expect("Failed cleaning up test database");
		}
	}

	pub async fn setup() -> TestContext{
		TEST_SETUP_DONE.get_or_init(|| async {
			teardown_test_dbs().await;
			let boostrap_res = bootstrap::bootstrap::Bootstrap::deploy(bootstrap::bootstrap::BootstrapMode::Test).await.unwrap();
			Io::init().await;
		}).await;
		let ctx = TestContext{
			users_repo: UsersRepo::new(DBDrivers::Postgres)
		};
		ctx
	}
}