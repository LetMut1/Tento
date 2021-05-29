use crate::diesel_component::schema::public::pre_confirmed_application_user;
use crate::entity::entity::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "pre_confirmed_application_user"]
pub struct Insert<'outer_a> {
    id: &'outer_a Uuid,
    email: &'outer_a str
}

impl<'outer_a> Insert<'outer_a> {
    pub fn new(pre_confirmed_application_user: &'outer_a PreConfirmedApplicationUser) -> Self {
        return Self {
            id: pre_confirmed_application_user.get_id().get_value().get_value(),
            email: pre_confirmed_application_user.get_email().get_value()
        };
    }
}
