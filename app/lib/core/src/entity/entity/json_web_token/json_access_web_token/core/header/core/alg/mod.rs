use std::default::Default;

pub struct Alg {            // TODO Возможность создавать экземпляр здесь не нужна ?
    value: &'static str
}

impl<'this> Alg {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn get_value(&'this self) -> String {
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