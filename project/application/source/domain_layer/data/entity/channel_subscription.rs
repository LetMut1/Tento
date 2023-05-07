use std::marker::PhantomData;
use super::application_user::ApplicationUser_Id;
use super::channel::Channel_Id;

pub use self::CreatedAt as ChannelSubscription_CreatedAt;

pub struct CreatedAt(String);

impl CreatedAt {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

pub struct ChannelSubscription {
    application_user_id: i64,
    _application_user_id: PhantomData<ApplicationUser_Id>,

    channel_id: i64,
    _channel_id: PhantomData<Channel_Id>,

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