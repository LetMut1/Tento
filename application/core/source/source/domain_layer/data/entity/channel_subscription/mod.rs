mod field;
use {
    self::field::CreatedAt,
    super::{
        channel::Channel_Id,
        user::User_Id,
    },
    std::marker::PhantomData,
};
pub struct ChannelSubscription {
    user__id: PhantomData<(i64, User_Id)>,
    channel__id: PhantomData<(i64, Channel_Id)>,
    created_at: PhantomData<(i64, CreatedAt)>,
}
pub type ChannelSubscription_CreatedAt = CreatedAt;
