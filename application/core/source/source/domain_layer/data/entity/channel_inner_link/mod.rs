mod field;
use {
    self::field::CreatedAt,
    super::channel::Channel_Id,
};
pub struct ChannelInnerLink {
    from: Channel_Id,
    to: Channel_Id,
    created_at: CreatedAt,
}
impl ChannelInnerLink {
    pub const MAXIMUM_QUANTITY: i16 = 10;
}
