use crate::domain_layer::entity::application_user_pre_confirmed::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use diesel::PgConnection as Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn create<'outer_a>(
        connection: &'outer_a Connection,
        application_user_pre_confirmed: &'outer_a ApplicationUserPreConfirmed
    ) -> Result<(), Self::Error>;

    fn delete<'outer_a>(
        connection: &'outer_a Connection,
        application_user_pre_confirmed: &'outer_a ApplicationUserPreConfirmed
    ) -> Result<(), Self::Error>;
}