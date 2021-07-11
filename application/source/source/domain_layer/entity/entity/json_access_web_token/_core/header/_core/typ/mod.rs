pub struct Typ {
    value: &'static str
}

impl Typ {
    pub const fn new() -> Self {
        return Self { 
            value: "JWT"
        };
    }

    pub fn get_value<'this>(&'this self) -> &'static str {
        return self.value
    }
}