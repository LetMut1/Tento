use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::entity::core::date_time::DateTime;

pub struct CreatedAt {
    value: DateTime
}

impl<'this, 'outer_a: 'this> CreatedAt {
    pub fn new() -> Self {
        return Self {
            value: DateTime::new()
        };
    }

    pub fn new_from_date_time(chrono_date_time: ChronoDateTime<Utc>) -> Self {
        return Self {
            value: DateTime::new_from_date_time(chrono_date_time)
        };
    }

    pub fn new_from_str(date_time: &'outer_a str) -> Self {
        return Self::new_from_date_time(ChronoDateTime::parse_from_rfc3339(date_time).unwrap().with_timezone(&Utc)); // TODO выбрасывать ошибку
    }

    pub fn get_value(&'this self) -> &'this DateTime {
        return &self.value;
    }
}