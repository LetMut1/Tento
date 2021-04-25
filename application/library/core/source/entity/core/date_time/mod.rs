use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use std::clone::Clone;

#[derive(Clone)]
pub struct DateTime {
    value: ChronoDateTime<Utc>
}

impl<'this, 'outer: 'this> DateTime {
    pub fn new() -> Self {
        return Self {
            value: Utc::now()
        };
    }

    pub fn new_from_date_time(value: ChronoDateTime<Utc>) -> Self {
        return Self {
            value
        };
    }

    pub fn new_from_str(value: &'outer str) -> Self {
        return Self {
            value: ChronoDateTime::parse_from_rfc3339(value).unwrap().with_timezone(&Utc)      // TODO выбрасывать ошибку
        };
    }

    pub fn get_value(&'this self) -> &'this ChronoDateTime<Utc> {
        return &self.value;
    }
}