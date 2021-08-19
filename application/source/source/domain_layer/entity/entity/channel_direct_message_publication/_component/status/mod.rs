pub struct Status {
    value: u8
}

impl Status {  // TODO заполнить константами тип контента
    pub fn new(value: u8) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value<'this>(&'this self) -> u8 {
        return self.value;
    }
}