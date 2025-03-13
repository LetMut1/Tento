mod field;
use {
    self::field::ExpiresAt,
    super::{
        channel::{
            Channel_Id,
            Channel_ObfuscationValue,
        },
        user::User_Id,
    },
};
// It is necessary to prevent User from receiving information about Channel using
// a bot-program that will perform ID enumeration.
pub struct ChannelToken {
    user__id: User_Id,
    channel__id: Channel_Id,
    channel__obfuscation_value: Channel_ObfuscationValue,
    expires_at: ExpiresAt,
}
pub type ChannelToken_ExpiresAt = ExpiresAt;
