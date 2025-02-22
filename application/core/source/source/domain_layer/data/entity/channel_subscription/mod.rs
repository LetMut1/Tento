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
    user__id: i64,
    _user__id: PhantomData<User_Id>,
    channel__id: i64,
    _channel__id: PhantomData<Channel_Id>,
    created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}
pub type ChannelSubscription_CreatedAt = CreatedAt;
