pub struct Typ {
    value: &'static str
}

impl<'this> Typ {
    pub const fn new() -> Self {
        return Self { 
            value: "JWT"
        };
    }

    pub fn get_value(&'this self) -> &'static str {
        return self.value
    }
}