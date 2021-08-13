pub struct ViewingQuantity {
    value: i64
}

impl ViewingQuantity {
    pub fn new(value: i64) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value<'this>(&'this self) -> i64 {
        return self.value;
    }
}