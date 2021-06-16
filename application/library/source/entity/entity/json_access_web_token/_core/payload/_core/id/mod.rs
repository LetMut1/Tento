use crate::entity::_core::uuid_v4::UuidV4;
use crate::error::main_error::main_error::MainError;
use std::clone::Clone;

#[derive(Clone)]
pub struct Id {
    value: UuidV4
}

impl Id {
    pub fn new() -> Self {
        return Self {
            value: UuidV4::new()
        };
    }

    pub fn new_from_string(uuid: String) -> Result<Self, MainError> {
        return Ok(
            Self {
                value: UuidV4::new_from_string(uuid)?
            }
        );
    }

    pub fn to_string<'this>(&'this self) -> String {
        return self.value.to_string();
    }

    pub fn get_value<'this>(&'this self) -> &'this UuidV4 {
        return &self.value;
    }
}