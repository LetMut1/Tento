use maybe_owned::MaybeOwned;

pub struct DeviceId<'outer> {
    value: MaybeOwned<'outer, String>
}

impl<'this, 'outer: 'this> DeviceId<'outer> {
    pub fn new(value: MaybeOwned<'outer, String>) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value(&'this self) -> &'this String {
        return &self.value;
    }
}