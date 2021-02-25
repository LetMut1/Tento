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