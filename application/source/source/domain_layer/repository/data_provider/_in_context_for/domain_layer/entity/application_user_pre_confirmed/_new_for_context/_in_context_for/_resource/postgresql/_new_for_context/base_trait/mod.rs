use crate::domain_layer::entity::application_user_pre_confirmed::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use diesel::PgConnection as Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn is_exist_by_application_user_email<'outer_a>(
        connection: &'outer_a Connection,
        application_user_email: &'outer_a str
    ) -> Result<bool, Self::Error>;

    fn get_by_application_user_email<'outer_a>(
        connection: &'outer_a Connection,
        application_user_email: &'outer_a str
    ) -> Result<Option<ApplicationUserPreConfirmed>, Self::Error>;
}