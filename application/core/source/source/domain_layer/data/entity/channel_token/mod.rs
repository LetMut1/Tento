mod field;
use {
    self::field::{
        ExpiresAt,
        ObfuscationValue,
        IsUserTheChannelSubscriber,
        IsUserTheChannelOwner,
    },
    super::{
        channel::Channel_Id,
        user::User_Id,
    },
};
// It is necessary to prevent User from receiving information about Channel using
// a bot-program that will perform ID enumeration.
pub struct ChannelToken {
    user__id: User_Id,
    channel__id: Channel_Id,
    obfuscation_value: ObfuscationValue,
    expires_at: ExpiresAt,
    is_user_the_channel_subscriber: IsUserTheChannelSubscriber,
    is_user_the_channel_owner: IsUserTheChannelOwner,
}
pub type ChannelToken_ObfuscationValue = ObfuscationValue;
pub type ChannelToken_ExpiresAt = ExpiresAt;
