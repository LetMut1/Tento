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

    pub created_at: CreatedAt,
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
pub struct Description;

impl Description {
    pub const MAXIMUM_LENGTH: usize = 500;
}

#[derive(Serialize, Deserialize)]
pub struct AccessModifier;

impl AccessModifier {
    pub const OPEN: i16 = 0;
    pub const CLOSE: i16 = 1;
}

pub enum AccessModifier_ {
    Open = AccessModifier::OPEN as isize,
    Close = AccessModifier::CLOSE as isize,
}

#[derive(Serialize, Deserialize)]
pub struct VisabilityModifier;

impl VisabilityModifier {
    pub const PUBLIC: i16 = 0;
    pub const PRIVATE: i16 = 1;
}

pub enum VisabilityModifier_ {
    Public = VisabilityModifier::PUBLIC as isize,
    Private = VisabilityModifier::PRIVATE as isize,
}

#[derive(Serialize, Deserialize)]
pub struct Orientation;

#[derive(Serialize, Deserialize)]
pub struct CoverImagePath;

#[derive(Serialize, Deserialize)]
pub struct BackgroundImagePath;

#[derive(Serialize, Deserialize)]
pub struct SubscribersQuantity;

#[derive(Serialize, Deserialize)]
pub struct MarksQuantity;

#[derive(Serialize, Deserialize)]
pub struct ViewingQuantity;

pub struct CreatedAt(pub String);