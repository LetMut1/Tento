pub struct EntertainingSeeableOnlyContentQuantity {
    value: i64
}

impl EntertainingSeeableOnlyContentQuantity {
    pub fn new(value: i64) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value<'this>(&'this self) -> i64 {
        return self.value;
    }
}