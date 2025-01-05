use axum::http::StatusCode;
use macros::DisplayViaDebug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{operation::operation::{ImResponse, OpResult, Operation}, ws_server::{im_message::{ChatMessage, MessageProtocol, MessageStatus}, messaging_service::send_message_to_user}};

// #[derive(Serialize, Deserialize)]
// pub struct SendMessageState{
// 	message: MessageProtocol 
// }

#[derive(Serialize, Deserialize)]
pub struct SendMessageOperation{
	state: ChatMessage,
	
    
}
#[derive(Serialize, Deserialize, Debug, DisplayViaDebug, Clone)]
#[serde(rename_all="camelCase")]
pub struct SendMessageOperationResult{
	pub message_status: MessageStatus,
	pub client_message_id: Uuid
}

impl Operation<SendMessageOperationResult, ChatMessage> for SendMessageOperation{
	fn new(state: ChatMessage) -> Self {
		SendMessageOperation{
			state
		}
	}

	fn state(&self) -> ChatMessage {
		self.state.clone()
	}

	async fn execute(&mut self) -> OpResult<ImResponse<SendMessageOperationResult>> {
		send_message_to_user(&MessageProtocol::Chat(self.state()), self.state.target).await?;
		Ok(ImResponse{
    		status: StatusCode::OK,
    		body: SendMessageOperationResult{
				client_message_id: self.state.client_request_id,
				message_status: MessageStatus::Sent
			},
})}
}

// #[cfg(test)]
// mod tests{
// }
