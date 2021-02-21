pub struct Confirmed {
    value: bool
}

impl<'a> Confirmed {
    pub fn new(value: bool) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value(&'a self) -> bool {
        return self.value;
    }
}