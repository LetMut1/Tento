use crate::error::main_error_kind::core::invalid_argument_error::InvalidArgumentError;
use uuid::Uuid;
use std::clone::Clone;

#[derive(Clone)]
pub struct UuidV4 {
    value: Uuid
}

impl<'this, 'outer: 'this> UuidV4 {
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

    pub fn new_from_string(value: String) -> Result<Self, InvalidArgumentError> {
        if let Ok(uuid) = Uuid::parse_str(value.as_str()) {
            return Ok(
                Self { 
                    value: uuid
                }
            );
        }

        return Err(InvalidArgumentError);
    }

    pub fn get_value(&'this self) -> &'this Uuid {
        return &self.value;
    }
}