mod field;
use {
    self::field::{
        ExpiresAt,
        ObfuscationValue,
    },
    super::{
        channel_publication1::ChannelPublication1_Id,
        channel_publication1_commentary::ChannelPublication1Commentary_Id,
        user::User_Id,
        channel::Channel_Id,
    },
};
// It is necessary to prevent User from creation of ChannelPublication1CommentaryMark on ChannelPublication1Commentary using
// a bot-program that will perform ID enumeration and to decrease requests quantity to database.
pub struct ChannelPublication1CommentaryToken {
    user__id: User_Id,
    channel__id: Channel_Id,
    channel_publication1__id: ChannelPublication1_Id,
    channel_publication1_commentary__id: ChannelPublication1Commentary_Id,
    obfuscation_value: ObfuscationValue,
    expires_at: ExpiresAt,
    commentary_author: User_Id,
}
pub type ChannelPublication1CommentaryToken_ExpiresAt = ExpiresAt;
pub type ChannelPublication1CommentaryToken_ObfuscationValue = ObfuscationValue;
