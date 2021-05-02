pub struct WrongEnterTriesQuanity {
    value: u8
}

impl<'this> WrongEnterTriesQuanity {
    pub fn new(value: u8) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value(&'this self) -> u8 {
        return self.value;
    }

    pub fn increment(&'this mut self) -> &'this mut Self {
        self.value = self.value + 1;

        return self;
    }
}