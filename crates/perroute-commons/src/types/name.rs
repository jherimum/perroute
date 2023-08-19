use std::borrow::Cow;

use validator::ValidationError;

pub fn validate(str: &str) -> Result<(), ValidationError> {
    if str.len() < 1 || str.len() > 255 {
        return Err(ValidationError {
            code: Cow::Borrowed("name"),
            message: Some(Cow::Borrowed("Invalid name")),
            params: Default::default(),
        });
    }
    Ok(())
}
