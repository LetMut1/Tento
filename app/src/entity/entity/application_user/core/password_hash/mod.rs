use maybe_owned::MaybeOwned;
use std::borrow::Borrow;

pub struct PasswordHash<'a> {
    value: MaybeOwned<'a, String>
}

impl<'a> PasswordHash<'a> {
    pub fn new(value: MaybeOwned<'a, String>) -> Self {     // TODO create Hash with Util HAher
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