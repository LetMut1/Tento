use http::request::Parts;

pub struct Base<T> {
    parts: Parts,
    convertible_data: T
}

impl<T> Base<T> {
    pub fn new(
        parts: Parts,
        convertible_data: T
    ) -> Self {
        return Self {
            parts,
            convertible_data
        };
    }

    pub fn into_inner(
        self
    ) -> (Parts, T) {
        return (
            self.parts,
            self.convertible_data
        );
    }
}