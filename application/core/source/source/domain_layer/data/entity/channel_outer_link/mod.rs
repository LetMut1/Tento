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
    from: PhantomData<(i64, Channel_Id)>,
    alias: PhantomData<(String, Alias)>,
    address: PhantomData<(String, Address)>,
    created_at: PhantomData<(i64, CreatedAt)>,
}
impl ChannelOuterLink {
    pub const MAXIMUM_QUANTITY: i16 = 5;
}
pub type ChannelOuterLink_Address = Address;
pub type ChannelOuterLink_Alias = Alias;
pub type ChannelOuterLink_CreatedAt = CreatedAt;
