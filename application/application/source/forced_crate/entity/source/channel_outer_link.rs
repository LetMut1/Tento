use super::channel::Channel_Id;
use serde::Serialize;
use serde::Deserialize;
use std::marker::PhantomData;

pub use self::Address as ChannelOuterLink_Address;
pub use self::Alias as ChannelOuterLink_Alias;
pub use self::CreatedAt as ChannelOuterLink_CreatedAt;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Alias(pub String);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Address(pub String);

pub struct CreatedAt(pub String);

pub struct ChannelOuterLink {
    pub from: i64,
    _from: PhantomData<Channel_Id>,

    pub alias: Alias,
    pub address: Address,
    pub created_at: CreatedAt,
}

impl ChannelOuterLink {
    pub const MAXIMUM_QUANTITY: i16 = 5;

    pub fn new(
        from: i64,
        alias: Alias,
        address: Address,
        created_at: CreatedAt,
    ) -> Self {
        return Self {
            from,
            _from: PhantomData,
            alias,
            address,
            created_at,
        }
    }
}
