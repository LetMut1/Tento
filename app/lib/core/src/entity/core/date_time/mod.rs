use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use maybe_owned::MaybeOwned;

pub struct DateTime<'outer> {
    value: MaybeOwned<'outer, ChronoDateTime<Utc>>
}

impl<'this, 'outer: 'this> DateTime<'outer> {
    pub fn new() -> Self {
        return Self {
            value: MaybeOwned::Owned(Utc::now())
        };
    }

    pub fn new_from_date_time(value: MaybeOwned<'outer, ChronoDateTime<Utc>>) -> Self {
        return Self {
            value
        };
    }

    pub fn new_from_string(value: &'outer String) -> Self {
        return Self {
            value:MaybeOwned::Owned(ChronoDateTime::parse_from_rfc3339(value.as_str()).unwrap().with_timezone(&Utc))        // TODO выбрасывать ошибку
        };
    }

    pub fn get_value(&'this self) -> &'this ChronoDateTime<Utc> {
        return &self.value;
    }
}