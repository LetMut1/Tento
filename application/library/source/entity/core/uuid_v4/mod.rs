use crate::error::main_error::core::invalid_argument_error::InvalidArgumentError;
use uuid::Uuid;
use std::clone::Clone;

#[derive(Clone)]
pub struct UuidV4 {
    value: Uuid
}

impl UuidV4 {
    pub fn new() -> Self {
        return Self {
            value: Uuid::new_v4()
        };
    }

    pub fn new_from_uuid(value: Uuid) -> Self {
        return Self {
            value
        };
    }

    pub fn new_from_string(uuid: String) -> Result<Self, InvalidArgumentError> {
        if let Ok(uuid) = Uuid::parse_str(uuid.as_str()) {
            return Ok(
                Self { 
                    value: uuid
                }
            );
        }

        return Err(InvalidArgumentError);
    }

    pub fn to_string<'this>(&'this self) -> String {
        return self.value.to_string();
    }

    pub fn get_value<'this>(&'this self) -> &'this Uuid {
        return &self.value;
    }
}