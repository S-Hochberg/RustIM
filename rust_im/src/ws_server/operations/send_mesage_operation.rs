use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{operation::operation::{ImResponse, OpResult, Operation}, ws_server::{im_message::{ChatMessage, MessageProtocol}, messaging_service::send_message_to_user}};

// #[derive(Serialize, Deserialize)]
// pub struct SendMessageState{
// 	message: MessageProtocol 
// }

#[derive(Serialize, Deserialize)]
pub struct SendMessageOperation{
	state: ChatMessage
    
}

impl Operation<(), ChatMessage> for SendMessageOperation{
	fn new(state: ChatMessage) -> Self {
		SendMessageOperation{
			state
		}
	}

	fn state(&self) -> ChatMessage {
		self.state.clone()
	}

	async fn execute(&mut self) -> OpResult<ImResponse<()>> {
		send_message_to_user(&MessageProtocol::Chat(self.state()), self.state.target).await?;
		Ok(ImResponse{
    		status: StatusCode::OK,
    		body: (),
})}
}

// #[cfg(test)]
// mod tests{
// }
