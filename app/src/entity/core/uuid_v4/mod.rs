use maybe_owned::MaybeOwned;
use std::borrow::Borrow;
use uuid::Uuid;

pub struct UuidV4<'a> {
    value: MaybeOwned<'a, Uuid>
}

impl<'a> UuidV4<'a> {
    pub fn new() -> Self {
        return Self {value: MaybeOwned::Owned(Uuid::new_v4())};
    }

    pub fn new_from(value: MaybeOwned<'a, Uuid>) -> Self {
        return Self {value};
    }

    pub fn set_value(&'a mut self, value: MaybeOwned<'a, Uuid>) -> &'a mut Self {
        self.value = value;

        return self;
    }

    pub fn get_value(&'a self) -> &'a Uuid {
        return self.value.borrow();
    }
}