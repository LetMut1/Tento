use std::default::Default;

pub struct Alg {
    value: &'static str
}

impl<'a> Alg {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn get_value(&'a self) -> String {
        return self.value.to_string()
    }
}

impl Default for Alg {
    fn default() -> Self {
        return Self { 
            value: "HS512" 
        };
    }
}