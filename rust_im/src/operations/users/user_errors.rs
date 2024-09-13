use std::fmt::{Display, Debug};

use axum::http::StatusCode;

use crate::operations::operation::{DefaultState, OpError};

pub enum UserErrors<State: Display + Debug>{
	DuplicateUser(String, State)
}

impl<State> Into<OpError<State>> for UserErrors<State>
where State: Display + Debug
{
	fn into(self) -> OpError<State> {
		match self {
			UserErrors::DuplicateUser(message, state) => OpError{message: format!("User already exists - {message}"), status: StatusCode::BAD_REQUEST, state},
		}		
	}
}