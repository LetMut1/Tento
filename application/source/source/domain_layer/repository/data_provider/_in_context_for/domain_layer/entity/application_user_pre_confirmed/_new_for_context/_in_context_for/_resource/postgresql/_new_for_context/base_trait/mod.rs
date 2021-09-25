use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use postgres::Client as Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn is_exist_by_application_user_email<'a>(
        connection: &'a mut Connection,
        application_user_email: &'a str
    ) -> Result<bool, Self::Error>;

    fn find_by_application_user_email<'a>(
        connection: &'a mut Connection,
        application_user_email: &'a str
    ) -> Result<Option<ApplicationUserPreConfirmed>, Self::Error>;
}