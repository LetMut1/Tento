use std::marker::PhantomData;
pub struct Id(PhantomData<i64>);
pub struct Text(PhantomData<Option<String>>);
pub struct MarksQuantity(PhantomData<i64>);
pub struct CreatedAt(PhantomData<i64>);
pub struct IsPredeleted(PhantomData<bool>);
pub struct CanBeDeletedFrom(PhantomData<i64>);
// TODO TODO TODO
impl CanBeDeletedFrom {
    pub const QUANTITY_OF_MICROSECONDS_BEFORE_DELETION: i64 = 1;
    const _GUARD: () = {
        static_assertions::const_assert!(
            CanBeDeletedFrom::QUANTITY_OF_MICROSECONDS_BEFORE_DELETION > 0
        );
        ()
    };
}
