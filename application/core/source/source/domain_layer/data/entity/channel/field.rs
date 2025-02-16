pub struct Id;
pub struct Name;
impl Name {
    pub const MAXIMUM_LENGTH: usize = 75;
}
pub struct LinkedName;
pub struct Description;
impl Description {
    pub const MAXIMUM_LENGTH: usize = 500;
}
#[repr(i16)]
pub enum AccessModifier {
    Open = 0,
    Close = 1,
}
#[repr(i16)]
pub enum VisabilityModifier {
    Public,
    Private,
}
pub struct Orientation;
pub struct CoverImagePath;
pub struct BackgroundImagePath;
pub struct SubscribersQuantity;
pub struct MarksQuantity;
pub struct ViewingQuantity;
pub struct ObfuscationValue;
pub struct CreatedAt;
