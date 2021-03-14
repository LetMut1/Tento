use std::clone::Clone;

pub struct Password {
    value: String
}

impl<'this> Password {
    pub fn new(value: String) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value(&'this self) -> &'this str {
        return self.value.as_str();
    }
}

impl Clone for Password {
    fn clone(&self) -> Self {
        panic!("It shouldn't be cloned");
    }
}