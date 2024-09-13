use std::{collections::HashMap, fmt::{Debug, Display}};

use axum::http::StatusCode;

use crate::{models::user::user::User, operations::operation::{DefaultState, OpError}};

struct UserErrors{
	pub DuplicateUser: OpError
}
pub const USER_ERRORS: UserErrors = UserErrors{
	DuplicateUser: OpError{message: "Duplicate user".to_string(), status: StatusCode::BAD_REQUEST, state: None }
};