use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
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

    pub fn new_from_str<'outer_a>(date_time: &'outer_a str) -> Result<Self, BaseError> {
        match ChronoDateTime::parse_from_rfc3339(date_time) {
            Ok(chrono_date_time) => {
                return Ok(Self::new_from_date_time(chrono_date_time.with_timezone(&Utc)));
            }
            Err(_) => {
                return Err(BaseError::InvalidArgumentError);
            }
        }
    }

    pub fn get_value<'this>(&'this self) -> &'this ChronoDateTime<Utc> {
        return &self.value;
    }
}