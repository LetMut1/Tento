use std::marker::PhantomData;
pub struct Id(PhantomData<i64>);
pub struct Name(PhantomData<String>);
impl Name {
    pub const MAXIMUM_LENGTH: usize = 75;
}
pub struct LinkedName(PhantomData<String>);
pub struct Description(PhantomData<Option<String>>);
impl Description {
    pub const MAXIMUM_LENGTH: usize = 500;
}
pub struct AccessModifier(PhantomData<i16>);
#[repr(i16)]
pub enum AccessModifier_ {
    Open = 0,
    Close = 1,
}
pub struct VisabilityModifier(PhantomData<i16>);
#[repr(i16)]
pub enum VisabilityModifier_ {
    Public,
    Private,
}
pub struct Orientation(PhantomData<Vec<i16>>);
pub struct CoverImagePath(PhantomData<Option<String>>);
pub struct BackgroundImagePath(PhantomData<Option<String>>);
pub struct SubscribersQuantity(PhantomData<i64>);
pub struct MarksQuantity(PhantomData<i64>);
pub struct ViewingQuantity(PhantomData<i64>);
pub struct ObfuscationValue(PhantomData<i64>);
pub struct CreatedAt(PhantomData<i64>);
