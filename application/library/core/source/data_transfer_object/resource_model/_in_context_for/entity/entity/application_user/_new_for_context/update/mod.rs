use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::diesel_component::schema::public::application_user;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::utility::_in_context_for::data_transfer_object::resource_model::_in_context_for::entity::entity::application_user::_new_for_context::update::_new_for_context::update_resolver::UpdateResolver;
use diesel::AsChangeset;

#[derive(AsChangeset)]
#[table_name = "application_user"]
pub struct Update<'outer> {
    email: Option<&'outer str>,
    nickname: Option<&'outer str>,
    password_hash: Option<&'outer str>,
    created_at: Option<&'outer ChronoDateTime<Utc>>
}

impl<'outer> Update<'outer> {
    pub fn new(application_user: &'outer ApplicationUser<'outer>, update_resolver: UpdateResolver) -> Self {
        let mut email: Option<&'outer str> = None;
        let mut nickname: Option<&'outer str> = None;
        let mut password_hash: Option<&'outer str> = None;
        let mut created_at: Option<&'outer ChronoDateTime<Utc>> = None;

        if update_resolver.is_change_email() {
            email = Some(application_user.get_email().get_value());
        }

        if update_resolver.is_change_nickname() {
            nickname = Some(application_user.get_nickname().get_value());
        }

        if update_resolver.is_change_password_hash() {
            password_hash = Some(application_user.get_passord_hash().get_value());
        }

        if update_resolver.is_change_created_at() {
            created_at = Some(application_user.get_created_at().get_value());
        }

        return Self {
            email,
            nickname,
            password_hash,
            created_at,
        };
    }
}
