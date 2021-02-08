use maybe_owned::MaybeOwned;

pub struct Email<'b> {
    value: MaybeOwned<'b, String>
}

impl<'a, 'b: 'a> Email<'b> {
    pub fn new(value: MaybeOwned<'b, String>) -> Self {
        return Self {
            value
        };
    }

    pub fn set_value(&'a mut self, value: MaybeOwned<'b, String>) -> &'a mut Self {
        self.value = value;

        return self;
    }

    pub fn get_value(&'a self) -> &'a String {
        return &self.value;
    }
}