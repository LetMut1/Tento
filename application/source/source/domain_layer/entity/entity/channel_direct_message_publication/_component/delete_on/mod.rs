use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::domain_layer::entity::proxed_type::date_time::DateTime;

pub struct DeleteOn {
    value: DateTime
}

impl DeleteOn {
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

    pub fn get_value<'this>(&'this self) -> &'this DateTime {
        return &self.value;
    }
}