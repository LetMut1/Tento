pub struct IsEntertaining {
    value: bool
}

impl IsEntertaining {
    pub fn new(value: bool) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value<'this>(&'this self) -> bool {
        return self.value;
    }
}