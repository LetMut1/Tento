use super::application_user::ApplicationUser_Id;
use super::channel::Channel_Id;

pub use self::CreatedAt as ChannelSubscription_CreatedAt;

pub struct CreatedAt(pub String);

pub struct ChannelSubscription {
    pub application_user_id: ApplicationUser_Id,
    pub channel_id: Channel_Id,
    pub created_at: CreatedAt,
}
