// What the fuck am i supposed to do here
use std::convert::Infallible;
use std::{collections::HashMap, future::Future, process::Output};
use axum::{handler::Handler, http::Method, routing::MethodRouter, routing::get, response::Response,response::IntoResponse, Extension};

use axum::routing::method_routing;
pub fn gettest<H, T, S>(handler: H) -> ()
where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static{

	}
pub struct RouteDefinition{
	method: Method,
	// handler: , //dyn Future<Output=Response>
	path: &'static str,
}
pub struct Controller{
	base_route: &'static str,
	routes: &'static [&'static RouteDefinition]
}

pub static USERS_CONTROLLER: Controller = Controller{
	base_route: "123",
	routes: &[&RouteDefinition{method: Method::GET, path: "/users"}]
};
async fn get_user()-> impl IntoResponse{
	"123".into_response()
}
async fn test(){
	// let x = get(get_user);
}
