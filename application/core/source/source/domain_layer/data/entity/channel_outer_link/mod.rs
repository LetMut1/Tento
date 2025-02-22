mod field;
use {
    self::field::{
        Address,
        Alias,
        CreatedAt,
    },
    super::channel::Channel_Id,
    std::marker::PhantomData,
};
pub struct ChannelOuterLink {
    from: i64,
    _from: PhantomData<Channel_Id>,
    alias: String,
    _alias: PhantomData<Alias>,
    address: String,
    _address: PhantomData<Address>,
    created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}
impl ChannelOuterLink {
    pub const MAXIMUM_QUANTITY: i16 = 5;
}
pub type ChannelOuterLink_Address = Address;
pub type ChannelOuterLink_Alias = Alias;
pub type ChannelOuterLink_CreatedAt = CreatedAt;
