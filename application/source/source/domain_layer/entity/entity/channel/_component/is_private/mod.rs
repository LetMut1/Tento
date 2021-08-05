pub struct IsPrivate {
    value: bool
}

impl IsPrivate {
    pub fn new(value: bool) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value<'this>(&'this self) -> bool {
        return self.value;
    }
}