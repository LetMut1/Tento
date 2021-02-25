use maybe_owned::MaybeOwned;
use std::convert::TryInto;
use uuid::Uuid;

pub struct UuidV4<'b> {
    value: MaybeOwned<'b, Uuid>
}

impl<'a, 'b: 'a> UuidV4<'b> {
    pub fn new() -> Self {
        return Self {
            value: MaybeOwned::Owned(Uuid::new_v4())
        };
    }

    pub fn new_from_uuid(value: &'b Uuid) -> Self {
        return Self {
            value: MaybeOwned::Borrowed(value)
        };
    }

    pub fn new_from_string(value: &'b String) -> Self {
        let value_bytes: &'_ [u8] = value.as_bytes();
        if value_bytes.len() == 16 {
            return Self { 
                value: MaybeOwned::Owned(Uuid::from_bytes(value_bytes.try_into().unwrap())) // TODO ВЫбрасывать ошибку тоже
            }
        } else {
            panic!("выбрасывать Ошибки"); // TODO 
        }
    }

    pub fn get_value(&'a self) -> &'a Uuid {
        return &self.value;
    }
}