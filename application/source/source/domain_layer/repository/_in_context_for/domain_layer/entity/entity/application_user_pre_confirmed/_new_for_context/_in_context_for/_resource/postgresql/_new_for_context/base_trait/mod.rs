use crate::domain_layer::entity::entity::application_user_pre_confirmed::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::entity::entity::application_user::_component::email::Email;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use diesel::PgConnection as Connection;

pub trait BaseTrait {
    fn create<'outer_a>(
        connection: &'outer_a Connection, application_user_pre_confirmed: &'outer_a ApplicationUserPreConfirmed
    ) -> Result<(), BaseError>;

    fn delete<'outer_a>(
        connection: &'outer_a Connection, application_user_pre_confirmed: &'outer_a ApplicationUserPreConfirmed
    ) -> Result<(), BaseError>;

    fn is_exist_by_application_user_email<'outer_a>(
        connection: &'outer_a Connection, application_user_email: &'outer_a Email
    ) -> Result<bool, BaseError>;

    fn get_by_application_user_email<'outer_a>(
        connection: &'outer_a Connection, application_user_email: &'outer_a Email
    ) -> Result<Option<ApplicationUserPreConfirmed>, BaseError>;
}