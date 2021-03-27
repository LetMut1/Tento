use std::clone::Clone;

#[derive(Clone)]
pub struct Value {
    value: String
}

impl<'this> Value {
    pub fn new(value: String) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value(&'this self) -> &'this str {
        return self.value.as_str();
    }
}