use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn create<'a>(
        connection: &'a mut Connection, 
        application_user_pre_confirmed: &'a ApplicationUserPreConfirmed,
        quantity_of_minutes_for_expiration: Option<u16>
    ) -> Result<(), Self::Error>;

    fn delete<'a>(
        connection: &'a mut Connection, 
        application_user_email: &'a str
    ) -> Result<(), Self::Error>;
}