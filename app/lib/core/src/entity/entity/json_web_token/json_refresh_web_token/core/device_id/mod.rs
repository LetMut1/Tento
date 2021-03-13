use std::clone::Clone;

pub struct DeviceId {
    value: String
}

impl<'this> DeviceId {
    pub fn new(value: String) -> Self {
        return Self {
            value
        };
    }

    pub fn get_value(&'this self) -> &'this str {
        return self.value.as_str();
    }
}

impl Clone for DeviceId {
    fn clone(&self) -> Self {
        panic!("It shouldn't be cloned");
    }
}