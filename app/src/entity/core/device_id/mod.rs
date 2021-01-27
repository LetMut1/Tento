use maybe_owned::MaybeOwned;
use std::borrow::Borrow;

pub struct DeviceId<'a> {
    value: MaybeOwned<'a, String>
}

impl<'a> DeviceId<'a> {
    pub fn new(value: MaybeOwned<'a, String>) -> Self {
        return Self {value};
    }

    pub fn set_value(&'a mut self, value: MaybeOwned<'a, String>) -> &'a mut Self {
        self.value = value;

        return self;
    }

    pub fn get_value(&'a self) -> &'a String {
        return self.value.borrow();
    }
}