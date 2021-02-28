pub struct Alg;

impl<'this> Alg {
    const VALUE: &'static str = "HS512";

    pub fn new() -> Self {
        return Self;
    }

    pub fn get_value(&'this self) -> String {
        return Self::VALUE.to_string()
    }
}