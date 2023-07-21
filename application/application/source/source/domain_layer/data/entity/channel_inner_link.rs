use super::channel::Channel_Id;

pub use self::CreatedAt as ChannelInnerLink_CreatedAt;

pub struct CreatedAt(String);

impl CreatedAt {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

pub struct ChannelInnerLink {
    pub from: Channel_Id,
    pub to: Channel_Id,
    pub created_at: CreatedAt,
}

impl ChannelInnerLink {
    pub const MAXIMUM_QUANTITY: i16 = 10;
}
