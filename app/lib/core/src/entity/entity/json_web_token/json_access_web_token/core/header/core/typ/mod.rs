use std::default::Default;

pub struct Typ {
    value: &'static str
}

impl<'this> Typ {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn get_value(&'this self) -> String {
        return self.value.to_string()
    }
}

impl Default for Typ {
    fn default() -> Self {
        return Self { 
            value: "JWT" 
        };
    }
}