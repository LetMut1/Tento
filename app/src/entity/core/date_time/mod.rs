use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use maybe_owned::MaybeOwned;

pub struct DateTime<'a> {
    value: MaybeOwned<'a, ChronoDateTime<Utc>>
}

impl<'a> DateTime<'a> {
    pub fn new() -> Self {
        return Self {
            value: MaybeOwned::Owned(Utc::now())
        };
    }

    pub fn new_from(value: MaybeOwned<'a, ChronoDateTime<Utc>>) -> Self {
        return Self {
            value
        };
    }

    pub fn set_value(&'a mut self, value: MaybeOwned<'a, ChronoDateTime<Utc>>) -> &'a mut Self {
        self.value = value;

        return self;
    }

    pub fn get_value(&'a self) -> &'a ChronoDateTime<Utc> {
        return &self.value;
    }
}