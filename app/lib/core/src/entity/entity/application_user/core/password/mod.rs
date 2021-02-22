use maybe_owned::MaybeOwned;

pub struct Password<'b> {
    value: MaybeOwned<'b, String>
}

impl<'a, 'b: 'a> Password<'b> {
    pub fn new(value: MaybeOwned<'b, String>) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value(&'a self) -> &'a String {
        return &self.value;
    }
}