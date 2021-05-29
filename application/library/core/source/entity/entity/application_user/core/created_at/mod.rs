use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::entity::core::date_time::DateTime;

pub struct CreatedAt {
    value: DateTime
}

impl CreatedAt {
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

    pub fn new_from_str<'outer_a>(date_time: &'outer_a str) -> Self {
        return Self {
            value: DateTime::new_from_str(date_time)
        }
    }

    pub fn to_string<'this>(&'this self) -> String {
        return self.value.get_value().to_rfc3339();
    }

    pub fn get_value<'this>(&'this self) -> &'this DateTime {
        return &self.value;
    }
}