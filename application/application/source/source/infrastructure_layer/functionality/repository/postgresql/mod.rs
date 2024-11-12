mod channel;
mod channel_inner_link;
mod channel_outer_link;
mod channel_subscription;
mod common;
mod user;
mod user_access_refresh_token;
mod user_authorization_token;
mod user_device;
mod user_registration_token;
mod user_reset_password_token;
pub use self::{
    channel::{
        By1 as ChannelBy1,
        By2 as ChannelBy2,
        By3 as ChannelBy3,
        Insert1 as ChannelInsert1,
    },
    channel_inner_link::{
        By1 as ChannelInnerLinkBy1,
        Insert1 as ChannelInnerLinkInsert1,
    },
    channel_outer_link::{
        By1 as ChannelOuterLinkBy1,
        Insert1 as ChannelOuterLinkInsert1,
    },
    channel_subscription::{
        By1 as ChannelSubscriptionBy1,
        Insert1 as ChannelSubscriptionInsert1,
    },
    common::{
        By1 as CommonBy1,
        By2 as CommonBy2,
        By3 as CommonBy3,
    },
    user::{
        By1 as UserBy1,
        By2 as UserBy2,
        By3 as UserBy3,
        Insert1 as UserInsert1,
        Update1 as UserUpdate1,
    },
    user_access_refresh_token::{
        By1 as UserAccessRefreshTokenBy1,
        By2 as UserAccessRefreshTokenBy2,
        Insert1 as UserAccessRefreshTokenInsert1,
        Update1 as UserAccessRefreshTokenUpdate1,
    },
    user_authorization_token::{
        By1 as UserAuthorizationTokenBy1,
        Insert1 as UserAuthorizationTokenInsert1,
        Update1 as UserAuthorizationTokenUpdate1,
        Update2 as UserAuthorizationTokenUpdate2,
        Update3 as UserAuthorizationTokenUpdate3,
        Update4 as UserAuthorizationTokenUpdate4,
    },
    user_device::Insert1 as UserDeviceInsert1,
    user_registration_token::{
        By1 as UserRegistrationTokenBy1,
        Insert1 as UserRegistrationTokenInsert1,
        Update1 as UserRegistrationTokenUpdate1,
        Update2 as UserRegistrationTokenUpdate2,
        Update3 as UserRegistrationTokenUpdate3,
        Update4 as UserRegistrationTokenUpdate4,
        Update5 as UserRegistrationTokenUpdate5,
    },
    user_reset_password_token::{
        By1 as UserResetPasswordTokenBy1,
        Insert1 as UserResetPasswordTokenInsert1,
        Update1 as UserResetPasswordTokenUpdate1,
        Update2 as UserResetPasswordTokenUpdate2,
        Update3 as UserResetPasswordTokenUpdate3,
        Update4 as UserResetPasswordTokenUpdate4,
        Update5 as UserResetPasswordTokenUpdate5,
    }
};
use std::marker::PhantomData;
pub struct Postgresql<E> {
    _entity: PhantomData<E>,
}
