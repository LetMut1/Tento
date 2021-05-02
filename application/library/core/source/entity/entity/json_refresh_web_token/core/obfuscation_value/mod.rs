pub struct ObfuscationValue {
    value: String
}

impl<'this> ObfuscationValue {
    pub fn new(value: String) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value(&'this self) -> &'this str {
        return self.value.as_str();
    }
}