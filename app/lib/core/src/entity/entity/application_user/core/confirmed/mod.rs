use std::clone::Clone;

pub struct Confirmed {
    value: bool
}

impl<'this> Confirmed {
    pub fn new(value: bool) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value(&'this self) -> bool {
        return self.value;
    }
}

impl Clone for Confirmed {
    fn clone(&self) -> Self {
        panic!("It shouldn't be cloned");
    }
}