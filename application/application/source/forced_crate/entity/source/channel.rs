use super::application_user::ApplicationUser_Id;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::marker::PhantomData;
pub use self::AccessModifier as Channel_AccessModifier;
pub use self::AccessModifier_ as Channel_AccessModifier_;
pub use self::BackgroundImagePath as Channel_BackgroundImagePath;
pub use self::CoverImagePath as Channel_CoverImagePath;
pub use self::CreatedAt as Channel_CreatedAt;
pub use self::Description as Channel_Description;
pub use self::Id as Channel_Id;
pub use self::LinkedName as Channel_LinkedName;
pub use self::MarksQuantity as Channel_MarksQuantity;
pub use self::Name as Channel_Name;
pub use self::Orientation as Channel_Orientation;
pub use self::SubscribersQuantity as Channel_SubscribersQuantity;
pub use self::ViewingQuantity as Channel_ViewingQuantity;
pub use self::VisabilityModifier as Channel_VisabilityModifier;
pub use self::VisabilityModifier_ as Channel_VisabilityModifier_;

pub struct Channel<'a> {
    pub id: i64,
    _id: PhantomData<Id>,

    pub owner: i64,
    _owner: PhantomData<ApplicationUser_Id>,

    pub name: Cow<'a, str>,
    _name: PhantomData<Name>,

    pub linked_name: String,
    _linked_name: PhantomData<LinkedName>,

    pub description: Option<Description>,
    pub access_modifier: AccessModifier,
    pub visability_modifier: VisabilityModifier,
    pub orientation: Orientation,
    pub cover_image_path: Option<CoverImagePath>,
    pub background_image_path: Option<BackgroundImagePath>,
    pub subscribers_quantity: SubscribersQuantity,
    pub marks_quantity: MarksQuantity,
    pub viewing_quantity: ViewingQuantity,
    pub created_at: CreatedAt,
}

impl<'a> Channel<'a> {
    pub fn new(
        id: i64,
        owner: i64,
        name: Cow<'a, str>,
        linked_name: String,
        description: Option<Description>,
        access_modifier: AccessModifier,
        visability_modifier: VisabilityModifier,
        orientation: Orientation,
        cover_image_path: Option<CoverImagePath>,
        background_image_path: Option<BackgroundImagePath>,
        subscribers_quantity: SubscribersQuantity,
        marks_quantity: MarksQuantity,
        viewing_quantity: ViewingQuantity,
        created_at: CreatedAt,
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
            access_modifier,
            visability_modifier,
            orientation,
            cover_image_path,
            background_image_path,
            subscribers_quantity,
            marks_quantity,
            viewing_quantity,
            created_at,
        };
    }
}

#[derive(Serialize, Deserialize)]
pub struct Id;

#[derive(Serialize, Deserialize)]
pub struct Name;

impl Name {
    pub const MAXIMUM_LENGTH: usize = 75;
}

#[derive(Serialize, Deserialize)]
pub struct LinkedName;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Description(pub String);

impl Description {
    pub const MAXIMUM_LENGTH: usize = 500;
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccessModifier(pub i16);

impl AccessModifier {
    pub const OPEN: i16 = 0;
    pub const CLOSE: i16 = 1;
}

pub enum AccessModifier_ {
    Open = AccessModifier::OPEN as isize,
    Close = AccessModifier::CLOSE as isize,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VisabilityModifier(pub i16);

impl VisabilityModifier {
    pub const PUBLIC: i16 = 0;
    pub const PRIVATE: i16 = 1;
}

pub enum VisabilityModifier_ {
    Public = VisabilityModifier::PUBLIC as isize,
    Private = VisabilityModifier::PRIVATE as isize,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Orientation(pub Vec<i16>);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct CoverImagePath(pub String);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct BackgroundImagePath(pub String);

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SubscribersQuantity(pub i64);

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MarksQuantity(pub i64);

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ViewingQuantity(pub i64);

pub struct CreatedAt(pub String);