use tracing::Level;


#[macro_export]
macro_rules! env_var {
    ( $var_name: expr, $default: expr ) => {
		{
			match option_env!($var_name){
				Some(var) => {
					var
				},
				None => {
					$default
				}
			}
		}
    };
	( $var_name: expr) => {
		{
			match option_env!($var_name){
				Some(var) => {
					Some(var)
				},
				None => {
					None
				}
			}
		}
    };
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
			tracing_level: match env_var!("TRACING_LEVEL"){
				Some("trace") => Level::TRACE,
				Some("debug") => Level::DEBUG,
				Some("info") => Level::INFO,
				Some("warn") => Level::WARN,
				Some("error") => Level::ERROR,
				_ => Level::INFO
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
    		deploy_bootstrap: env_var!("DEPLOY_BOOTSTRAP", "true") == "true"
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
pub struct PostgresConfig{
	pub db_url: &'static str,
	pub db_name: &'static str,
	pub users_table: &'static str
}
impl Configuration for PostgresConfig{
	fn new() -> Self {
		PostgresConfig{
    		db_url: env_var!("DATABASE_URL", "postgresql://postgres:password@localhost:5432"),
    		db_name: env_var!("DB_NAME", "rust_im"),
    		users_table: env_var!("USERS_TABLE", "users"),
}
	}
}