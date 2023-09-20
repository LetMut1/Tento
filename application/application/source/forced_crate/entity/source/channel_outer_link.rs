use super::channel::Channel_Id;
use serde::Serialize;
use serde::Deserialize;

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
    pub from: Channel_Id,
    pub alias: Alias,
    pub address: Address,
    pub created_at: CreatedAt,
}

impl ChannelOuterLink {
    pub const MAXIMUM_QUANTITY: i16 = 5;
}
