use maybe_owned::MaybeOwned;
use std::convert::TryInto;
use uuid::Uuid;

pub struct UuidV4<'outer> {
    value: MaybeOwned<'outer, Uuid>
}

impl<'this, 'outer: 'this> UuidV4<'outer> {
    pub fn new() -> Self {
        return Self {
            value: MaybeOwned::Owned(Uuid::new_v4())
        };
    }

    pub fn new_from_uuid(value: &'outer Uuid) -> Self {
        return Self {
            value: MaybeOwned::Borrowed(value)
        };
    }

    pub fn new_from_string(value: &'outer String) -> Self {
        let value_bytes: &'_ [u8] = value.as_bytes();

        if value_bytes.len() == 16 {
            return Self { 
                value: MaybeOwned::Owned(Uuid::from_bytes(value_bytes.try_into().unwrap())) // TODO ВЫбрасывать ошибку 
            }
        } else {
            panic!("выбрасывать Ошибки"); // TODO 
        }
    }

    pub fn get_value(&'this self) -> &'this Uuid {
        return &self.value;
    }
}