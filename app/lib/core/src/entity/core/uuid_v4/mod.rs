use std::clone::Clone;
use std::convert::TryInto;
use uuid::Uuid;

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

    pub fn new_from_str(value: &'outer str) -> Self {
        let value_bytes: &'_ [u8] = value.as_bytes();

        if value_bytes.len() == 16 {
            return Self { 
                value: Uuid::from_bytes(value_bytes.try_into().unwrap()) // TODO ВЫбрасывать ошибку 
            }
        } else {
            panic!("выбрасывать Ошибки"); // TODO 
        }
    }

    pub fn get_value(&'this self) -> &'this Uuid {
        return &self.value;
    }
}

impl Clone for UuidV4 {
    fn clone(&self) -> Self {
        panic!("It shouldn't be cloned");
    }
}