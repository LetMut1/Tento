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
    from: Channel_Id,
    to: Channel_Id,
    created_at: CreatedAt,
}

impl ChannelInnerLink {
    pub const MAXIMUM_QUANTITY: i16 = 10;

    pub fn new(from: Channel_Id, to: Channel_Id, created_at: CreatedAt) -> Self {
        return Self {
            from,
            to,
            created_at,
        };
    }
}
