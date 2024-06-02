use super::application_user::ApplicationUser_Id;
use super::channel::Channel_Id;
use std::marker::PhantomData;

pub use self::CreatedAt as ChannelSubscription_CreatedAt;

pub struct ChannelSubscription {
    pub application_user_id: i64,
    _application_user_id: PhantomData<ApplicationUser_Id>,

    pub channel_id: i64,
    _channel_id: PhantomData<Channel_Id>,

    pub created_at: CreatedAt,
}

impl ChannelSubscription {
    pub fn new(
        application_user_id: i64,
        channel_id: i64,
        created_at: CreatedAt,
    ) -> Self {
        return Self {
            application_user_id,
            _application_user_id: PhantomData,
            channel_id,
            _channel_id: PhantomData,
            created_at,
        };
    }
}

pub struct CreatedAt(pub String);