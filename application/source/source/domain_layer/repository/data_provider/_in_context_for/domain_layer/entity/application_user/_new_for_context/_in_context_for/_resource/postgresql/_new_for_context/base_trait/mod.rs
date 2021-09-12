use crate::domain_layer::entity::application_user::ApplicationUser;
use diesel::PgConnection as Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn is_exist_by_nickanme<'outer_a>(
        connection: &'outer_a Connection,
        nickname: &'outer_a str
    ) -> Result<bool, Self::Error>;

    fn is_exist_by_email<'outer_a>(
        connection: &'outer_a Connection,
        email: &'outer_a str
    ) -> Result<bool, Self::Error>;

    fn get_by_email<'outer_a>(
        connection: &'outer_a Connection,
        email: &'outer_a str
    ) -> Result<Option<ApplicationUser<'static>>, Self::Error>;

    fn get_by_id<'outer_a>(
        connection: &'outer_a Connection,
        id: &'outer_a i64
    ) -> Result<Option<ApplicationUser<'static>>, Self::Error>;
}