use std::clone::Clone;

#[derive(Clone)]
pub struct Id {
    value: i64
}

impl Id {
    pub fn new(value: i64) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value<'this>(&'this self) -> i64 {
        return self.value;
    }
}