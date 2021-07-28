pub struct Password {
    value: String
}

impl Password {
    pub fn new(value: String) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value<'this>(&'this self) -> &'this str {
        return self.value.as_str();
    }
}