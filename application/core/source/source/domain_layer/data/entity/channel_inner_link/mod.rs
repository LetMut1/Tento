mod field;
use {
    self::field::CreatedAt,
    super::channel::Channel_Id,
    std::marker::PhantomData,
};
pub struct ChannelInnerLink {
    from: PhantomData<(i64, Channel_Id)>,
    to: PhantomData<(i64, Channel_Id)>,
    created_at: PhantomData<(i64, CreatedAt)>,
}
impl ChannelInnerLink {
    pub const MAXIMUM_QUANTITY: i16 = 10;
}
pub type ChannelInnerLink_CreatedAt = CreatedAt;
