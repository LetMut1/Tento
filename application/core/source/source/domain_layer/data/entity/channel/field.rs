use std::marker::PhantomData;
pub struct Id(PhantomData<i64>);
pub struct Name(PhantomData<String>);
impl Name {
    pub const MAXIMUM_LENGTH: usize = 75;
}
pub struct LinkedName(PhantomData<String>);
pub struct Description(PhantomData<String>);
impl Description {
    pub const MAXIMUM_LENGTH: usize = 500;
}
pub struct AccessModifier(PhantomData<u8>);
#[repr(u8)]
pub enum AccessModifier_ {
    Open = 0,
    Close = 1,
}
pub struct VisabilityModifier(PhantomData<u8>);
#[repr(u8)]
pub enum VisabilityModifier_ {
    Public = 0,
    Private = 1,
}
pub struct CoverImagePath(PhantomData<String>);
pub struct BackgroundImagePath(PhantomData<String>);
pub struct SubscribersQuantity(PhantomData<u32>);
pub struct CreatedAt(PhantomData<i64>);
