use std::clone::Clone;

#[derive(Clone)]
pub struct Email {
    value: String
}

impl Email {
    pub fn new(value: String) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value<'this>(&'this self) -> &'this str {
        return self.value.as_str();
    }
}