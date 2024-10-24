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
pub enum AccessModifier {
    Open,
    Close,
}
impl AccessModifier {
    pub const CLOSE: i16 = 1;
    pub const OPEN: i16 = 0;
    pub fn from_representation(access_modifier: AccessModifier) -> i16 {
        return match access_modifier {
            Self::Open => Self::OPEN,
            Self::Close => Self::CLOSE,
        };
    }
    pub fn to_representation(access_modifier: i16) -> AccessModifier {
        return if access_modifier == Self::OPEN {
            Self::Open
        } else {
            Self::Close
        };
    }
}
pub enum VisabilityModifier {
    Public,
    Private,
}
impl VisabilityModifier {
    pub const PRIVATE: i16 = 1;
    pub const PUBLIC: i16 = 0;
    pub fn from_representation(visability_modifier: VisabilityModifier) -> i16 {
        return match visability_modifier {
            Self::Public => Self::PUBLIC,
            Self::Private => Self::PRIVATE,
        };
    }
    pub fn to_representation(visability_modifier: i16) -> VisabilityModifier {
        return if visability_modifier == Self::PUBLIC {
            Self::Public
        } else {
            Self::Private
        };
    }
}
pub struct Orientation;
pub struct CoverImagePath;
pub struct BackgroundImagePath;
pub struct SubscribersQuantity;
pub struct MarksQuantity;
pub struct ViewingQuantity;
pub struct CreatedAt;
