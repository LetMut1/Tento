mod field;
use self::field::{
    Address,
    Alias,
    CreatedAt,
};
use super::channel::Channel_Id;
use std::marker::PhantomData;
pub struct ChannelOuterLink {
    pub from: i64,
    _from: PhantomData<Channel_Id>,
    pub alias: String,
    _alias: PhantomData<Alias>,
    pub address: String,
    _address: PhantomData<Address>,
    pub created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}
impl ChannelOuterLink {
    pub const MAXIMUM_QUANTITY: i16 = 5;
    pub fn new(from: i64, alias: String, address: String, created_at: i64) -> Self {
        return Self {
            from,
            _from: PhantomData,
            alias,
            _alias: PhantomData,
            address,
            _address: PhantomData,
            created_at,
            _created_at: PhantomData,
        };
    }
}
pub type ChannelOuterLink_Address = Address;
pub type ChannelOuterLink_Alias = Alias;
pub type ChannelOuterLink_CreatedAt = CreatedAt;
