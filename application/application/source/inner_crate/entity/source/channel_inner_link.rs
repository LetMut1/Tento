use super::channel::Channel_Id;

pub use self::CreatedAt as ChannelInnerLink_CreatedAt;

pub struct CreatedAt(pub String);

pub struct ChannelInnerLink {
    pub from: Channel_Id,
    pub to: Channel_Id,
    pub created_at: CreatedAt,
}

impl ChannelInnerLink {
    pub const MAXIMUM_QUANTITY: i16 = 10;
}
