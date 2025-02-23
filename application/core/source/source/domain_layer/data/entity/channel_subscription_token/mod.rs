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
    user__id: PhantomData<(i64, User_Id)>,
    channel__id: PhantomData<(i64, Channel_Id)>,
    channel__obfuscation_value: PhantomData<(i64, Channel_ObfuscationValue)>,
    created_at: PhantomData<(i64, CreatedAt)>,
}
pub type ChannelSubscriptionToken_CreatedAt = CreatedAt;