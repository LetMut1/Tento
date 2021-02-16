use maybe_owned::MaybeOwned;

pub struct DeviceId<'b> {
    value: MaybeOwned<'b, String>
}

impl<'a, 'b: 'a> DeviceId<'b> {
    pub fn new(value: MaybeOwned<'b, String>) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value(&'a self) -> &'a String {
        return &self.value;
    }
}