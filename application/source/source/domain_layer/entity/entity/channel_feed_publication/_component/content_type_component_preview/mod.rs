pub struct ContentTypeComponentPreview {
    value: String
}

impl ContentTypeComponentPreview {
    pub fn new(value: String) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value<'this>(&'this self) -> &'this str {
        return self.value.as_str();
    }
}