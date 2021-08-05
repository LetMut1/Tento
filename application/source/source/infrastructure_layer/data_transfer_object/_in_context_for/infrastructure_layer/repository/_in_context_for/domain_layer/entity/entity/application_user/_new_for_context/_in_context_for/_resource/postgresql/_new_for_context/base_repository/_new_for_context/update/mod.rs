use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::infrastructure_layer::service::diesel_component::schema_describer::public::application_user;
use crate::domain_layer::entity::entity::application_user::application_user::ApplicationUser;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::_new_for_context::update::_new_for_context::update_resolver::UpdateResolver;
use diesel::AsChangeset;

#[derive(AsChangeset)]
#[table_name = "application_user"]
pub struct Update<'outer_a> {
    email: Option<&'outer_a str>,
    nickname: Option<&'outer_a str>,
    password_hash: Option<&'outer_a str>,
    created_at: Option<&'outer_a ChronoDateTime<Utc>>
}

impl<'outer_a> Update<'outer_a> {
    pub fn new(application_user: &'outer_a ApplicationUser<'_>, update_resolver: UpdateResolver) -> Self {
        let mut email: Option<&'outer_a str> = None;
        let mut nickname: Option<&'outer_a str> = None;
        let mut password_hash: Option<&'outer_a str> = None;
        let mut created_at: Option<&'outer_a ChronoDateTime<Utc>> = None;

        if update_resolver.is_change_email() {
            email = Some(application_user.get_email().get_value());
        }

        if update_resolver.is_change_nickname() {
            nickname = Some(application_user.get_nickname().get_value());
        }

        if update_resolver.is_change_password_hash() {
            password_hash = Some(application_user.get_password_hash().get_value());
        }

        if update_resolver.is_change_created_at() {
            created_at = Some(application_user.get_created_at().get_value().get_value());
        }

        return Self {
            email,
            nickname,
            password_hash,
            created_at,
        };
    }
}