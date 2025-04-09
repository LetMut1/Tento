mod field;
use {
    self::field::ExpiresAt,
    super::{
        channel_publication1::{
            ChannelPublication1_Id,
            ChannelPublication1_ObfuscationValue,
        },
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
    channel_publication1__obfuscation_value: ChannelPublication1_ObfuscationValue,
    expires_at: ExpiresAt,
}
pub type ChannelPublication1Token_ExpiresAt = ExpiresAt;
