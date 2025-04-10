mod field;
use {
    self::field::{
        ExpiresAt,
        ObfuscationValue,
    },
    super::{
        channel_publication1::ChannelPublication1_Id,
        user::User_Id,
        channel::Channel_Id,
    },
};
// It is necessary to prevent User from creation of ChannelPublication1Mark on ChannelPublication1 using
// a bot-program that will perform ID enumeration and to decrease requests quantity to database.
pub struct ChannelPublication1Token {
    user__id: User_Id,
    channel__id: Channel_Id,
    channel_publication1__id: ChannelPublication1_Id,
    obfuscation_value: ObfuscationValue,
    expires_at: ExpiresAt,
}
pub type ChannelPublication1Token_ExpiresAt = ExpiresAt;
pub type ChannelPublication1Token_ObfuscationValue = ObfuscationValue;
