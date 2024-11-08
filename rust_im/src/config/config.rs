use std::{env};

use tracing::Level;


#[macro_export]
macro_rules! env_var {
    ( $var_name: expr, $default: expr ) => {
		{
			env_var_with_default($var_name, $default)
		}
    };
	( $var_name: expr) => {
		{
			option_env_var($var_name)
		}
    };
}

pub fn env_var_with_default(name: &str, default: &'static str) -> String{
	match env::var(name) {
		Ok(var) => var,
		Err(_) => default.to_string(),
	}
}
pub fn option_env_var(name: &str) -> Option<String>{
	match env::var(name) {
		Ok(var) => Some(var),
		Err(_) => None,
	}
}
pub struct Config{
	pub db: DbConfig,
	pub bootstrap: BootstrapConfig,
	pub tracing_level: tracing::Level
}
pub trait Configuration{
	fn new() -> Self;
}

impl Configuration for Config{
	fn new() -> Self {
		Config{
			db: DbConfig::new(),
			bootstrap: BootstrapConfig::new(),
			tracing_level: match option_env_var("TRACING_LEVEL"){
				Some(val) => { match val.as_str() {
					"trace" => Level::TRACE,
					"debug" => Level::DEBUG,
					"info" => Level::INFO,
					"warn" => Level::WARN,
					"error" => Level::ERROR,
					_ => Level::DEBUG
				}
				}
				_ => Level::DEBUG
			}
		}
	}
}
pub struct BootstrapConfig{
	pub deploy_bootstrap: bool
}
impl Configuration for BootstrapConfig{
	fn new() -> Self {
		BootstrapConfig{
    		deploy_bootstrap: env_var_with_default("DEPLOY_BOOTSTRAP", "false") == "true"
}
	}
}
pub struct DbConfig{
	pub postgres: PostgresConfig
}
impl Configuration for DbConfig{
	fn new() -> Self {
		DbConfig{
			postgres: PostgresConfig::new()
		}
	}
}
pub struct PostgresConfig{}
impl PostgresConfig{
	pub fn db_url(&self) -> String{
		env_var!("DATABASE_URL", "postgresql://postgres:password@localhost:5432")
	}
	pub fn db_name(&self) -> String{
		env_var!("DB_NAME", "rust_im")
	}
	pub fn users_table(&self) -> String{
		env_var!("USERS_TABLE", "users")
	}

}
impl Configuration for PostgresConfig{
	fn new() -> Self {
		PostgresConfig{ }
	}
}