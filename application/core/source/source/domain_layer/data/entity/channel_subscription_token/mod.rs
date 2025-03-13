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
};
// It is necessary to prevent User from creation of ChannelSubscription on Channel using
// a bot-program that will perform ID enumeration.
pub struct ChannelSubscriptionToken {
    user__id: User_Id,
    channel__id: Channel_Id,
    channel__obfuscation_value: Channel_ObfuscationValue,
    expires_at: ExpiresAt,
}
pub type ChannelSubscriptionToken_ExpiresAt = ExpiresAt;