use crate::domain_layer::entity::application_user::ApplicationUser;
use postgres::Client as Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn is_exist_by_nickanme<'outer_a>(
        connection: &'outer_a mut Connection,
        nickname: &'outer_a str
    ) -> Result<bool, Self::Error>;

    fn is_exist_by_email<'outer_a>(
        connection: &'outer_a mut Connection,
        email: &'outer_a str
    ) -> Result<bool, Self::Error>;

    fn find_by_email<'outer_a>(
        connection: &'outer_a mut Connection,
        email: &'outer_a str
    ) -> Result<Option<ApplicationUser>, Self::Error>;

    fn find_by_id<'outer_a>(
        connection: &'outer_a mut Connection,
        id: &'outer_a i64
    ) -> Result<Option<ApplicationUser>, Self::Error>;
}