use crate::domain_layer::entity::entity::application_user::_core::email::Email;
use crate::domain_layer::entity::entity::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use diesel::PgConnection as Connection;

pub trait BaseRepositoryTrait {
    fn create<'outer_a>(
        connection: &'outer_a Connection, pre_confirmed_application_user: &'outer_a PreConfirmedApplicationUser
    ) -> Result<(), BaseError>;

    fn delete<'outer_a>(
        connection: &'outer_a Connection, pre_confirmed_application_user: &'outer_a PreConfirmedApplicationUser
    ) -> Result<(), BaseError>;

    fn is_exist_by_application_user_email<'outer_a>(
        connection: &'outer_a Connection, application_user_email: &'outer_a Email
    ) -> Result<bool, BaseError>;

    fn get_by_application_user_email<'outer_a>(
        connection: &'outer_a Connection, application_user_email: &'outer_a Email
    ) -> Result<Option<PreConfirmedApplicationUser>, BaseError>;
}