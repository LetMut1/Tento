mod field;
use {
    self::field::CreatedAt,
    super::{
        channel_publication1::ChannelPublication1_Id,
        user::User_Id,
    },
};
pub struct ChannelPublication1Mark {
    user__id: User_Id,
    channel_publication1__id: ChannelPublication1_Id,
    created_at: CreatedAt,
}
