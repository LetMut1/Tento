pub use self::AccessModifier as Channel_AccessModifier;
pub use self::Description as Channel_Description;
pub use self::Id as Channel_Id;
pub use self::LinkedName as Channel_LinkedName;
pub use self::Name as Channel_Name;
pub use self::Orientation as Channel_Orientation;
pub use self::VisabilityModifier as Channel_VisabilityModifier;
use super::application_user::ApplicationUser_Id;
use std::borrow::Cow;
use std::marker::PhantomData;
pub struct Channel<'a> {
    pub id: i64,
    _id: PhantomData<Id>,

    pub owner: i64,
    _owner: PhantomData<ApplicationUser_Id>,

    pub name: Cow<'a, str>,
    _name: PhantomData<Name>,

    pub linked_name: String,
    _linked_name: PhantomData<LinkedName>,

    pub description: Option<String>,
    _description: PhantomData<Description>,

    pub access_modifier: i16,
    _access_modifier: PhantomData<AccessModifier>,

    pub visability_modifier: i16,
    _visability_modifier: PhantomData<VisabilityModifier>,

    pub orientation: Vec<i16>,
    _orientation: PhantomData<Orientation>,

    pub cover_image_path: Option<String>,
    _cover_image_path: PhantomData<CoverImagePath>,

    pub background_image_path: Option<String>,
    _background_image_path: PhantomData<BackgroundImagePath>,

    pub subscribers_quantity: i64,
    _subscribers_quantity: PhantomData<SubscribersQuantity>,

    pub marks_quantity: i64,
    _marks_quantity: PhantomData<MarksQuantity>,

    pub viewing_quantity: i64,
    _viewing_quantity: PhantomData<ViewingQuantity>,

    pub created_at: String,
    _created_at: PhantomData<CreatedAt>,
}
impl<'a> Channel<'a> {
    pub fn new(
        id: i64,
        owner: i64,
        name: Cow<'a, str>,
        linked_name: String,
        description: Option<String>,
        access_modifier: i16,
        visability_modifier: i16,
        orientation: Vec<i16>,
        cover_image_path: Option<String>,
        background_image_path: Option<String>,
        subscribers_quantity: i64,
        marks_quantity: i64,
        viewing_quantity: i64,
        created_at: String,
    ) -> Self {
        return Self {
            id,
            _id: PhantomData,
            owner,
            _owner: PhantomData,
            name,
            _name: PhantomData,
            linked_name,
            _linked_name: PhantomData,
            description,
            _description: PhantomData,
            access_modifier,
            _access_modifier: PhantomData,
            visability_modifier,
            _visability_modifier: PhantomData,
            orientation,
            _orientation: PhantomData,
            cover_image_path,
            _cover_image_path: PhantomData,
            background_image_path,
            _background_image_path: PhantomData,
            subscribers_quantity,
            _subscribers_quantity: PhantomData,
            marks_quantity,
            _marks_quantity: PhantomData,
            viewing_quantity,
            _viewing_quantity: PhantomData,
            created_at,
            _created_at: PhantomData,
        };
    }
}
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
    pub const OPEN: i16 = 0;
    pub const CLOSE: i16 = 1;
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
    pub const PUBLIC: i16 = 0;
    pub const PRIVATE: i16 = 1;
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
