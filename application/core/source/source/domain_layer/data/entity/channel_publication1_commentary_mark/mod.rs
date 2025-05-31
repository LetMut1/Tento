mod field;
use {
    self::field::CreatedAt,
    super::{
        channel_publication1_commentary::ChannelPublication1Commentary_Id,
        user::User_Id,
    },
};
pub struct ChannelPublication1CommentaryMark {
    user__id: User_Id,
    channel_publication1_commenatry__id: ChannelPublication1Commentary_Id,
    created_at: CreatedAt,
}
