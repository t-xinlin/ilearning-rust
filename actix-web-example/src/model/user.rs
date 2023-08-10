use std::fmt::Debug;
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Course {
    pub id: Option<String>,
    #[validate(custom(function = "validate_username", message = "invalid name"))]
    pub name: Option<String>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}


fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("invalid name"));
    }
    Ok(())
}
