pub struct WrongEnterTriesQuanity {
    value: u8
}

impl WrongEnterTriesQuanity {
    pub fn new(value: u8) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value<'this>(&'this self) -> u8 {
        return self.value;
    }

    pub fn increment<'this>(&'this mut self) -> &'this mut Self {
        self.value = self.value + 1;

        return self;
    }
}