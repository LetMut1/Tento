use std::clone::Clone;

pub struct Email {
    value: String
}

impl<'this> Email {
    pub fn new(value: String) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value(&'this self) -> &'this str {
        return self.value.as_str();
    }
}

impl Clone for Email {
    fn clone(&self) -> Self {
        panic!("It shouldn't be cloned");
    }
}