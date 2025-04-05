use axum::http::StatusCode;
use chrono::Utc;
use macros::DisplayViaDebug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{operation::operation::{ImResponse, OpResult, Operation}, ws_server::im_message::{ImMessage, MessageRequest}, CONNECTION_MANAGER};

#[derive(Serialize, Deserialize, Debug, DisplayViaDebug, Clone)]
pub struct SendMessageRequest{
    pub request: MessageRequest,
    pub sender: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct SendMessageOperation{
	state: SendMessageRequest
}

impl Operation<(), SendMessageRequest> for SendMessageOperation{
	async fn execute(&mut self) -> OpResult<ImResponse<()>> {
        let im_message = ImMessage{
            message_type: self.state.request.message_type,
            content: self.state.request.content.clone(),
            sender: self.state.sender,
            recipient: self.state.request.recipient,
            conversation_type: self.state.request.conversation_type,
            sent_at: Utc::now(),
            recieved_at: None,
            updated_at: Utc::now(),
        };
        CONNECTION_MANAGER.send_message_to_user(im_message).await?;
		Ok(ImResponse{body: (), status: StatusCode::OK})
	}

	fn new(state: SendMessageRequest) -> Self {
			SendMessageOperation{
				state
			}
		}

	fn state(&self) -> SendMessageRequest {
			self.state.clone()
		}
}