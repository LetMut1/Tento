mod field;
use {
    self::field::{
        Address,
        Alias,
        CreatedAt,
    },
    super::channel::Channel_Id,
};
pub struct ChannelOuterLink {
    from: Channel_Id,
    alias: Alias,
    address: Address,
    created_at: CreatedAt,
}
impl ChannelOuterLink {
    pub const MAXIMUM_QUANTITY: i16 = 5;
}