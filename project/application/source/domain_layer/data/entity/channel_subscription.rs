use std::marker::PhantomData;
use super::application_user::ApplicationUser_Id;
use super::channel::Id as ChannelId;

pub struct ChannelSubscription {
    application_user_id: i64,
    _application_user_id: PhantomData<ApplicationUser_Id>,

    channel_id: i64,
    _channel_id: PhantomData<ChannelId>,

    created_at: String,
    _created_at: PhantomData<CreatedAt>
}

impl ChannelSubscription {
    pub fn new(
        application_user_id: i64,
        channel_id: i64,
        created_at: String
    ) -> Self {
        return Self {
            application_user_id,
            _application_user_id: PhantomData,
            channel_id,
            _channel_id: PhantomData,
            created_at,
            _created_at: PhantomData
        };
    }
}

pub struct CreatedAt;