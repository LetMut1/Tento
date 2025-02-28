mod field;
use {
    self::field::ExpiresAt,
    super::{
        user::User_Id,
        channel::{
            Channel_Id,
            Channel_ObfuscationValue,
        },
    },
    std::marker::PhantomData,
};
pub struct ChannelSubscriptionToken {
    user__id: PhantomData<(i64, User_Id)>,
    channel__id: PhantomData<(i64, Channel_Id)>,
    channel__obfuscation_value: PhantomData<(i64, Channel_ObfuscationValue)>,
    expires_at: PhantomData<(i64, ExpiresAt)>,
}
pub type ChannelSubscriptionToken_ExpiresAt = ExpiresAt;