use super::channel::Channel_Id;
use serde::Serialize;

#[cfg(feature = "manual_testing")]
use serde::Deserialize;

pub use self::Address as ChannelOuterLink_Address;
pub use self::Alias as ChannelOuterLink_Alias;
pub use self::CreatedAt as ChannelOuterLink_CreatedAt;

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(transparent)]
pub struct Alias(pub String);

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
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
