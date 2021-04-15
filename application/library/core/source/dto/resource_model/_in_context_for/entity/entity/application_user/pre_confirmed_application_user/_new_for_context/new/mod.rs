use crate::diesel_component::schema::public::pre_confirmed_application_user;
use crate::entity::entity::application_user::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "pre_confirmed_application_user"]
pub struct New<'outer> {
    id: &'outer Uuid,
    email: &'outer str
}

impl<'outer> New<'outer> {
    pub fn new(pre_confirmed_application_user: &'outer PreConfirmedApplicationUser) -> Self {
        return Self {
            id: pre_confirmed_application_user.get_id().get_value(),
            email: pre_confirmed_application_user.get_email().get_value()
        };
    }
}
