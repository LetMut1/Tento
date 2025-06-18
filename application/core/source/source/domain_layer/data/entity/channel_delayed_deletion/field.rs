use {
    crate::domain_layer::data::entity::channel_publication1_token::ChannelPublication1Token_ExpiresAt,
    std::marker::PhantomData,
};
pub struct CanBeDeletedFrom(PhantomData<i64>);
impl CanBeDeletedFrom {
    pub const QUANTITY_OF_MICROSECONDS_BEFORE_DELETION: i64 = 2 * ChannelPublication1Token_ExpiresAt::QUANTITY_OF_MICROSECONDS_FOR_EXPIRATION;
    const _GUARD: () = {
        static_assertions::const_assert!(
            CanBeDeletedFrom::QUANTITY_OF_MICROSECONDS_BEFORE_DELETION > ChannelPublication1Token_ExpiresAt::QUANTITY_OF_MICROSECONDS_FOR_EXPIRATION
        );
        ()
    };
}
pub struct CreatedAt(PhantomData<i64>);
