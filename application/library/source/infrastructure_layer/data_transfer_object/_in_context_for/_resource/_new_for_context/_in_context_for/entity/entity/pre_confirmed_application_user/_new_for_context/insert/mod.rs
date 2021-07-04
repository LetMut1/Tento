use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::infrastructure_layer::diesel_component::schema::public::pre_confirmed_application_user;
use crate::domain_layer::entity::entity::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use diesel::Insertable;

#[derive(Insertable)]
#[table_name = "pre_confirmed_application_user"]
pub struct Insert<'outer_a> {
    email: &'outer_a str,
    created_at: &'outer_a ChronoDateTime<Utc>
}

impl<'outer_a> Insert<'outer_a> {
    pub fn new(pre_confirmed_application_user: &'outer_a PreConfirmedApplicationUser) -> Self {
        return Self {
            email: pre_confirmed_application_user.get_email().get_value(),
            created_at: pre_confirmed_application_user.get_created_at().get_value().get_value()
        };
    }
}
