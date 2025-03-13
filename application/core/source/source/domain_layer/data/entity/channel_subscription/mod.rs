mod field;
use {
    self::field::CreatedAt,
    super::{
        channel::Channel_Id,
        user::User_Id,
    },
};
pub struct ChannelSubscription {
    user__id: User_Id,
    channel__id: Channel_Id,
    created_at: CreatedAt,
}
