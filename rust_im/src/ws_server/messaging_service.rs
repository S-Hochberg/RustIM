use axum::extract::ws::Message;
use futures_util::SinkExt;
use serde_json::to_string;
use tracing::warn;
use uuid::Uuid;
use anyhow::Result;
use crate::{operation::operation::{OpError, OpErrorInput}, CONNECTION_MANAGER};

use super::im_message::{MessageProtocol};

pub async fn send_message_to_user(message: &MessageProtocol, target: Uuid) -> Result<(), OpError> {
        match CONNECTION_MANAGER.get_connection(&target) {
            Some(mut connection) => {
                let ws_message = Message::Text(match to_string(message) {
                    Ok(stringified) => stringified,
                    Err(err) => {
                        warn!("Failed stringifying a message with error {err}");
                        Err(OpError::internal_error())?
                    },
                });
                match connection.send_channel.send(ws_message).await{
                    Ok(_) => Ok(()),
                    Err(error) => {
                        warn!("Failed sending protocol message {message} to target {target} with error {error}");
                        Err(OpError::internal_error())
                    },
                }
            },
            None => 
                Err(OpError::bad_request(OpErrorInput{
                     message: Some(String::from("Target user is not connected")),
                     status: None, 
                     state: None }))
            ,
        }
}