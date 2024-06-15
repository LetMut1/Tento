pub mod application_user;
pub mod application_user_access_refresh_token;
pub mod application_user_authorization_token;
pub mod application_user_device;
pub mod application_user_registration_token;
pub mod application_user_reset_password_token;
pub mod channel;
pub mod channel_inner_link;
pub mod channel_outer_link;
pub mod channel_subscription;
pub mod common;
use std::marker::PhantomData;
pub struct PostgresqlRepository<E> {
    _entity: PhantomData<E>,
}
