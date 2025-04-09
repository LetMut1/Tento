mod field;
use {
    self::field::{
        ObfuscationValue,
        ExpiresAt,
    },
    super::{
        channel::Channel_Id,
        user::User_Id,
    },
};
// It is necessary to prevent User from creation of ChannelSubscription on Channel using
// a bot-program that will perform ID enumeration.
pub struct ChannelSubscriptionToken {
    user__id: User_Id,
    channel__id: Channel_Id,
    obfuscation_value: ObfuscationValue,
    expires_at: ExpiresAt,
}
pub type ChannelSubscriptionToken_ExpiresAt = ExpiresAt;
pub type ChannelSubscriptionToken_ObfuscationValue = ObfuscationValue;
