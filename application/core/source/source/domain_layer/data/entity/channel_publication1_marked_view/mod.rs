mod field;
use {
    self::field::{
        CreatedAt,
        MarkedAt,
    },
    super::{
        channel_publication1::ChannelPublication1_Id,
        user::User_Id,
    },
};
pub struct ChannelPublication1MarkedView {
    user__id: User_Id,
    channel_publication1__id: ChannelPublication1_Id,
    marked_at: MarkedAt,
    created_at: CreatedAt,
}
