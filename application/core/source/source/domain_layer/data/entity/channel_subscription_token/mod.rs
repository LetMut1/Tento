mod field;
use {
    self::field::CreatedAt,
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
    user__id: i64,
    _user__id: PhantomData<User_Id>,
    channel__id: i64,
    _channel__id: PhantomData<Channel_Id>,
    channel__obfuscation_value: i64,
    _channel__obfuscation_value: PhantomData<Channel_ObfuscationValue>,
    created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}
pub type ChannelSubscriptionToken_CreatedAt = CreatedAt;