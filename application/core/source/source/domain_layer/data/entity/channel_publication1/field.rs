use {
    std::marker::PhantomData,
    crate::domain_layer::data::entity::channel_publication1_token::ChannelPublication1Token_ExpiresAt,
};
pub struct Id(PhantomData<i64>);
pub struct ImagesPathes(PhantomData<Vec<String>>);
pub struct Text(PhantomData<Option<String>>);
pub struct CommentariesQuantity(PhantomData<i64>);
pub struct MarksQuantity(PhantomData<i64>);
pub struct ViewQuantity(PhantomData<i64>);
pub struct ObfuscationValue(PhantomData<i64>);
pub struct CreatedAt(PhantomData<i64>);
pub struct IsPredeleted(PhantomData<bool>);
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