use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use std::clone::Clone;

#[derive(Clone)]
pub struct DateTime {
    value: ChronoDateTime<Utc>
}

impl DateTime {
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

    pub fn new_from_str<'outer_a>(date_time: &'outer_a str) -> Self {
        return Self::new_from_date_time(ChronoDateTime::parse_from_rfc3339(date_time).unwrap().with_timezone(&Utc));      // TODO выбрасывать ошибку
    }

    pub fn to_string<'this>(&'this self) -> String {
        return self.value.to_rfc3339();
    }

    pub fn get_value<'this>(&'this self) -> &'this ChronoDateTime<Utc> {
        return &self.value;
    }
}