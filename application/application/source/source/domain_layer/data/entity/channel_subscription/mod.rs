mod field;
use self::field::CreatedAt;
use super::{
    user::User_Id,
    channel::Channel_Id,
};
use std::marker::PhantomData;
pub struct ChannelSubscription {
    pub user__id: i64,
    _user__id: PhantomData<User_Id>,
    pub channel__id: i64,
    _channel__id: PhantomData<Channel_Id>,
    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}
impl ChannelSubscription {
    pub fn new(user__id: i64, channel__id: i64, created_at: i64) -> Self {
        return Self {
            user__id,
            _user__id: PhantomData,
            channel__id,
            _channel__id: PhantomData,
            created_at,
            _created_at: PhantomData,
        };
    }
}
pub type ChannelSubscription_CreatedAt = CreatedAt;