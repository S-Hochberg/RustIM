use std::sync::OnceLock;

use sqlx::PgPool;

pub struct NUOnceLock<T> {
	lock: OnceLock<T>,
	message: &'static str
}
impl <T> NUOnceLock<T> {
	pub const fn new(message: &'static str) -> NUOnceLock<T>{
		NUOnceLock{
			lock: OnceLock::new(),
			message
		}
	}
	pub fn get(&self) -> &T{
		self.lock.get().expect(&self.message)
	}
	pub fn set(&self, value: T) -> Result<(), T>{
		self.lock.set(value)
	}
	
}
use super::db::postgres::postgres::PostrgresConnection;
pub static IO: NUOnceLock<Io> = NUOnceLock::new("IO not initialized yet");
#[derive(Debug)]
pub struct Io{
	pub sql: PgPool
}
impl Io{
	pub async fn init() -> (){
		let io_instance = Io { sql: PostrgresConnection::pool().await };
		IO.set(io_instance).expect("Failed initializing IO");
	}
}
