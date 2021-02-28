use std::default::Default;

pub struct Typ {
    value: &'static str         // TODO Возможность создавать экземпляр здесь не нужна ?
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