pub struct Alg {
    value: &'static str
}

impl<'this> Alg {
    pub const fn new() -> Self {
        return Self { 
            value: "HS512" 
        };
    }

    pub fn get_value(&'this self) -> &'static str {
        return self.value;
    }
}