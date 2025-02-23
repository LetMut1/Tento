mod field;
use {
    self::field::CreatedAt,
    super::channel::Channel_Id,
    std::marker::PhantomData,
};
pub struct ChannelInnerLink {
    from: i64,
    _from: PhantomData<Channel_Id>,
    to: i64,
    _to: PhantomData<Channel_Id>,
    created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}
impl ChannelInnerLink {
    pub const MAXIMUM_QUANTITY: i16 = 10;
}
pub type ChannelInnerLink_CreatedAt = CreatedAt;
