use crate::domain_layer::entity::application_user::ApplicationUser;
use postgres::Client as ConnectionXXXxDelete;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::boxed::Box;
use tokio_postgres::Client as Connection;


pub trait BaseTraitXXXxDelete {
    type Error: Error;

    fn is_exist_by_nickanmeXXXxDelete<'a>(
        connection: &'a mut ConnectionXXXxDelete,
        nickname: &'a str
    ) -> Result<bool, Self::Error>;

    fn is_exist_by_emailXXXxDelete<'a>(
        connection: &'a mut ConnectionXXXxDelete,
        email: &'a str
    ) -> Result<bool, Self::Error>;

    fn find_by_emailXXXxDelete<'a>(
        connection: &'a mut ConnectionXXXxDelete,
        email: &'a str
    ) -> Result<Option<ApplicationUser>, Self::Error>;

    fn find_by_nicknameXXXxDelete<'a>(
        connection: &'a mut ConnectionXXXxDelete,
        nickname: &'a str
    ) -> Result<Option<ApplicationUser>, Self::Error>;

    fn find_by_idXXXxDelete<'a>(
        connection: &'a mut ConnectionXXXxDelete,
        id: &'a i64
    ) -> Result<Option<ApplicationUser>, Self::Error>;
}

pub trait BaseTrait {
    type Error: Error;

    fn is_exist_by_nickanme<'a>(
        connection: &'a mut Connection,
        nickname: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<bool, Self::Error>> + 'a>>;

    fn is_exist_by_email<'a>(
        connection: &'a mut Connection,
        email: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<bool, Self::Error>> + 'a>>;

    fn find_by_email<'a>(
        connection: &'a mut Connection,
        email: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Option<ApplicationUser>, Self::Error>> + 'a>>;

    fn find_by_nickname<'a>(
        connection: &'a mut Connection,
        nickname: &'a str
    ) -> Pin<Box<dyn Future<Output = Result<Option<ApplicationUser>, Self::Error>> + 'a>>;

    fn find_by_id<'a>(
        connection: &'a mut Connection,
        id: &'a i64
    ) -> Pin<Box<dyn Future<Output = Result<Option<ApplicationUser>, Self::Error>> + 'a>>;
}