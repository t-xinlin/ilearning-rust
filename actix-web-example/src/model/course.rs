use std::fmt::Debug;
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Course {
    pub id: Option<String>,
    #[validate(range(min = 1, max = 32))]
    pub teacher_id: i64,
    #[validate(custom(function = "validate_unique_username", message = "invalid name"))]
    pub name: Option<String>,
    pub time: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<f64>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl Course {
    pub fn new() -> Self {
        return Course {
            id: None,
            teacher_id: 0,
            name: None,
            time: None,
            description: None,
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: None,
            level: None,
        };
    }
}

fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("invalid name"));
    }
    Ok(())
}
