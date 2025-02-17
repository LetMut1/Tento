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
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct ChannelSubscriptionToken {
    pub user__id: i64,
    _user__id: PhantomData<User_Id>,
    pub channel__id: i64,
    _channel__id: PhantomData<Channel_Id>,
    pub channel__obfuscation_value: i64,
    _channel__obfuscation_value: PhantomData<Channel_ObfuscationValue>,
    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}
impl ChannelSubscriptionToken {
    pub fn new(
        user__id: i64,
        channel__id: i64,
        channel__obfuscation_value: i64,
        created_at: i64,
    ) -> Self {
        return Self {
           user__id,
           _user__id: PhantomData,
           channel__id,
           _channel__id: PhantomData,
           channel__obfuscation_value,
           _channel__obfuscation_value: PhantomData,
           created_at,
           _created_at: PhantomData,
        };
    }
}
pub type ChannelSubscriptionToken_CreatedAt = CreatedAt;