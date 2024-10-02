pub mod user;
pub mod user_access_refresh_token;
pub mod user_authorization_token;
pub mod user_device;
pub mod user_registration_token;
pub mod user_reset_password_token;
pub mod channel;
pub mod channel_inner_link;
pub mod channel_outer_link;
pub mod channel_subscription;
pub mod common;
use std::marker::PhantomData;
pub struct PostgresqlRepository<E> {
    _entity: PhantomData<E>,
}
