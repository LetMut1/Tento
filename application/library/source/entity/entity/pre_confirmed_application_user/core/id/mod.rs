use crate::entity::core::uuid_v4::UuidV4;
use crate::error::main_error::core::invalid_argument_error::InvalidArgumentError;
use uuid::Uuid;

pub struct Id {
    value: UuidV4
}

impl Id {
    pub fn new() -> Self {
        return Self {
            value: UuidV4::new()
        };
    }

    pub fn new_from_uuid(uuid: Uuid) -> Self {
        return Self {
            value: UuidV4::new_from_uuid(uuid)
        };
    }

    pub fn new_from_string(uuid: String) -> Result<Self, InvalidArgumentError> {
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