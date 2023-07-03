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
    application_user_id: ApplicationUser_Id,
    channel_id: Channel_Id,
    created_at: CreatedAt,
}

impl ChannelSubscription {
    pub fn new(
        application_user_id: ApplicationUser_Id,
        channel_id: Channel_Id,
        created_at: CreatedAt,
    ) -> Self {
        return Self {
            application_user_id,
            channel_id,
            created_at,
        };
    }
}
