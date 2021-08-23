use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::domain_layer::entity::entity::application_user_pre_confirmed::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::infrastructure_layer::service::diesel_component::schema_describer::public::pre_confirmed_application_user;
use diesel::Insertable;

#[derive(Insertable)]
#[table_name = "pre_confirmed_application_user"]
pub struct Insert<'outer_a> {
    email: &'outer_a str,
    created_at: &'outer_a ChronoDateTime<Utc>
}

impl<'outer_a> Insert<'outer_a> {
    pub fn new(application_user_pre_confirmed: &'outer_a ApplicationUserPreConfirmed) -> Self {
        return Self {
            email: application_user_pre_confirmed.get_email().get_value(),
            created_at: application_user_pre_confirmed.get_created_at().get_value().get_value()
        };
    }
}
