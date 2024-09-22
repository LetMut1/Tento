use super::{
    application_user::ApplicationUser_Id,
    channel::Channel_Id,
};
use std::marker::PhantomData;
pub struct ChannelSubscription {
    pub application_user__id: i64,
    _application_user__id: PhantomData<ApplicationUser_Id>,

    pub channel__id: i64,
    _channel__id: PhantomData<Channel_Id>,

    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}
impl ChannelSubscription {
    pub fn new(application_user__id: i64, channel__id: i64, created_at: i64) -> Self {
        return Self {
            application_user__id,
            _application_user__id: PhantomData,
            channel__id,
            _channel__id: PhantomData,
            created_at,
            _created_at: PhantomData,
        };
    }
}
pub struct CreatedAt;
