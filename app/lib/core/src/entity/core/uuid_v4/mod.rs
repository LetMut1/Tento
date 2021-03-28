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

    pub fn new_from_str(value: &'outer str) -> Result<Self, ()> {
        match Uuid::parse_str(value) {
            Ok(uuid) => {
                return Ok(
                    Self { 
                        value: uuid
                    }
                );
            },
            Err(error) => {
                return Err(());
            }
        };
    }

    pub fn get_value(&'this self) -> &'this Uuid {
        return &self.value;
    }
}