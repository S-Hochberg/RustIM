use axum::{routing::MethodRouter, Router};


pub struct InternalController{
	router: Router,
	base_path: &'static str
}
impl InternalController{
	pub fn new(base_path: &'static str) -> Self{
		InternalController{
			base_path,
			router: Router::new()
		}	
	}
	pub fn route(mut self, path: &str, method_router: MethodRouter) -> Self {
		let route = format!("{}{}", self.base_path, path);
		self.router = self.router.route(route.as_str(), method_router);
		self
	}
	pub fn get_router(self) -> Router{
		self.router
	}
}

pub trait Controller{
	fn new(base_path: &'static str) -> Self;
	fn get_router(self) -> Router;
}