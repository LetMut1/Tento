use std::marker::PhantomData;

pub struct Generator<S> {
    _subject: PhantomData<S>
}

pub struct NumberRow;

impl Generator<NumberRow> {
    pub fn generate_6() -> String {
        return "666666".to_string();        // TODOD format!("{}{}{}{}{}{}", rand....)
    }
}