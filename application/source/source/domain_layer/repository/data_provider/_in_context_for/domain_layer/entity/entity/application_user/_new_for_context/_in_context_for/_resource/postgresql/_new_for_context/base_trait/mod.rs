use crate::domain_layer::entity::entity::application_user::_component::email::Email;
use crate::domain_layer::entity::entity::application_user::_component::id::Id;
use crate::domain_layer::entity::entity::application_user::_component::nickname::Nickname;
use crate::domain_layer::entity::entity::application_user::application_user::ApplicationUser;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use diesel::PgConnection as Connection;

pub trait BaseTrait {
    fn is_exist_by_nickanme<'outer_a>(connection: &'outer_a Connection, nickname: &'outer_a Nickname) -> Result<bool, BaseError>;

    fn is_exist_by_email<'outer_a>(connection: &'outer_a Connection, email: &'outer_a Email) -> Result<bool, BaseError>;

    fn get_by_email<'outer_a>(connection: &'outer_a Connection, email: &'outer_a Email) -> Result<Option<ApplicationUser<'static>>, BaseError>;

    fn get_by_id<'outer_a>(connection: &'outer_a Connection, id: &'outer_a Id) -> Result<Option<ApplicationUser<'static>>, BaseError>;
}