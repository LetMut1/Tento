use super::channel::Channel_Id;
use serde::Serialize;
use serde::Deserialize;
use std::marker::PhantomData;

pub use self::Address as ChannelOuterLink_Address;
pub use self::Alias as ChannelOuterLink_Alias;
pub use self::CreatedAt as ChannelOuterLink_CreatedAt;

pub struct ChannelOuterLink {
    pub from: i64,
    _from: PhantomData<Channel_Id>,

    pub alias: String,
    _alias: PhantomData<Alias>,

    pub address: Address,
    pub created_at: CreatedAt,
}

impl ChannelOuterLink {
    pub const MAXIMUM_QUANTITY: i16 = 5;

    pub fn new(
        from: i64,
        alias: String,
        address: Address,
        created_at: CreatedAt,
    ) -> Self {
        return Self {
            from,
            _from: PhantomData,
            alias,
            _alias: PhantomData,
            address,
            created_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Alias;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Address(pub String);

pub struct CreatedAt(pub String);