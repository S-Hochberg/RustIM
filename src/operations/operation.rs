use axum::response::IntoResponse;

use crate::api::response::ImResponse;

pub trait Operation<Res: IntoResponse>{
	type State;
	fn new(s: Self::State) -> Self;
	async fn init(&mut self) -> (){}
	async fn validate(&mut self) -> (){}
	async fn authorize(&mut self) -> (){}
	async fn execute(&mut self) -> ImResponse<Res>;
}
pub struct OperationsExecutor{}
impl OperationsExecutor{
	async fn execute_op<T: IntoResponse>(mut op: impl Operation<T>) -> ImResponse<T>{
		op.init().await;
		op.init().await;
		op.init().await;
		op.execute().await
	} 
}