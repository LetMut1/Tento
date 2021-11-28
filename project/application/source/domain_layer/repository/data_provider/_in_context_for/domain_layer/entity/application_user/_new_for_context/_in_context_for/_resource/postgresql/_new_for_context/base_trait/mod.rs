use crate::domain_layer::entity::application_user::ApplicationUser;
use postgres::Client as Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn is_exist_by_nickanme<'a>(
        connection: &'a mut Connection,
        nickname: &'a str
    ) -> Result<bool, Self::Error>;

    fn is_exist_by_email<'a>(
        connection: &'a mut Connection,
        email: &'a str
    ) -> Result<bool, Self::Error>;

    fn find_by_email<'a>(
        connection: &'a mut Connection,
        email: &'a str
    ) -> Result<Option<ApplicationUser>, Self::Error>;

    fn find_by_id<'a>(
        connection: &'a mut Connection,
        id: &'a i64
    ) -> Result<Option<ApplicationUser>, Self::Error>;
}