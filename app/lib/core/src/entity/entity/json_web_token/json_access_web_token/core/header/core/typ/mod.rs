pub struct Typ;

impl<'this> Typ {
    const VALUE: &'static str = "JWT";

    pub fn new() -> Self {
        return Self;
    }

    pub fn get_value(&'this self) -> String {
        return Self::VALUE.to_string()
    }
}