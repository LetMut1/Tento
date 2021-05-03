use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::entity::core::date_time::DateTime;
use crate::utility::date_time_expiration_resolver::DateTimeExpirationResolver;
use std::clone::Clone;

#[derive(Clone)]
pub struct Exp {
    value: DateTime
}

impl<'this, 'outer_a: 'this> Exp {
    pub fn new() -> Self {
        return Self {
            value: DateTimeExpirationResolver::create_json_access_web_token_first()
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

    pub fn to_string(&'this self) -> String {
        return self.value.get_value().to_rfc3339();
    }
}