use maybe_owned::MaybeOwned;

pub struct PasswordHash<'b> {
    value: MaybeOwned<'b, String>
}

impl<'a, 'b: 'a> PasswordHash<'b> {
    pub fn new(value: MaybeOwned<'b, String>) -> Self {     // TODO create Hash with Util HAher
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