use axum::extract::ws::Message;
use futures_util::SinkExt;
use serde_json::to_string;
use tracing::{warn, error};
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

pub async fn send_error_to_user(error: OpError, user_id: Uuid) -> () {
    let error_message = MessageProtocol::Error(error);
    //TODO: handle failure of sending error message to client more cleanly (when would this even happen?)
    if let Err(err) = send_message_to_user(&error_message, user_id).await{
        match err.status{ 
            // StatusCode::INTERNAL_SERVER_ERROR => {
            //     error!("Failed sending server error - {error_message} to user {user_id}")
            // },
            //TODO: handle failure of sending message to client
            _ => {
                error!("Failed sending server error - {error_message} to user {user_id} with error {err}")
            }
        }
    };
    ()
}