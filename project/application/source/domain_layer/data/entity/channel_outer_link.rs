use super::channel::Channel_Id;

pub use self::Alias as ChannelOuterLink_Alias;
pub use self::Address as ChannelOuterLink_Address;
pub use self::CreatedAt as ChannelOuterLink_CreatedAt;

pub struct Alias(String);

impl Alias {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

pub struct Address(String);

impl Address {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

pub struct CreatedAt(String);

impl CreatedAt {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

pub struct ChannelOuterLink {
    from: Channel_Id,
    alias: Alias,
    address: Address,
    created_at: CreatedAt
}

impl ChannelOuterLink {
    pub const MAXIMUM_QUANTITY: i16 = 5;

    pub fn new(
        from: Channel_Id,
        alias: Alias,
        address: Address,
        created_at: CreatedAt
    ) -> Self {
        return Self {
            from,
            alias,
            address,
            created_at
        }
    }
}