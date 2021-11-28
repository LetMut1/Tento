use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use postgres::Client as Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn create<'a>(
        connection: &'a mut Connection,
        application_user_pre_confirmed: &'a ApplicationUserPreConfirmed
    ) -> Result<(), Self::Error>;

    fn delete<'a>(
        connection: &'a mut Connection,
        application_user_pre_confirmed: &'a ApplicationUserPreConfirmed
    ) -> Result<(), Self::Error>;
}