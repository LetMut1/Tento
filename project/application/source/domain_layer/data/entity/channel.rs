use std::borrow::Cow;
use std::marker::PhantomData;
use super::application_user::ApplicationUser_Id;

pub use self::Id as Channel_Id;
pub use self::Name as Channel_Name;
pub use self::LinkedName as Channel_LinkedName;
pub use self::Description as Channel_Description;
pub use self::AccessModifier as Channel_AccessModifier;
pub use self::VisabilityModifier as Channel_VisabilityModifier;
pub use self::Orientation as Channel_Orientation;
pub use self::CoverImagePath as Channel_CoverImagePath;
pub use self::BackgroundImagePath as Channel_BackgroundImagePath;
pub use self::SubscribersQuantity as Channel_SubscribersQuantity;
pub use self::MarksQuantity as Channel_MarksQuantity;
pub use self::ViewingQuantity as Channel_ViewingQuantity;
pub use self::CreatedAt as Channel_CreatedAt;

pub struct Id;

pub struct Name;

pub struct LinkedName;

pub struct Description;

pub enum AccessModifier {
    /// 0 in integer representation.
    Open,
    /// 1 in integer representation.
    Close
}

pub enum VisabilityModifier {
    /// 0 in integer representation.
    Public,
    /// 1 in integer representation.
    Private
}

pub struct Orientation;

pub struct CoverImagePath;

pub struct BackgroundImagePath;

pub struct SubscribersQuantity;

pub struct MarksQuantity;

pub struct ViewingQuantity;

pub struct CreatedAt;

pub struct Channel<'a> {
    id: i64,
    _id: PhantomData<Id>,

    owner: i64,
    _owner: PhantomData<ApplicationUser_Id>,

    name: Cow<'a, str>,
    _name: PhantomData<Name>,

    linked_name: String,
    _linked_name: PhantomData<LinkedName>,

    description: Option<String>,
    _description: PhantomData<Description>,

    access_modifier: i16,
    _access_modifier: PhantomData<AccessModifier>,

    visability_modifier: i16,
    _visability_modifier: PhantomData<VisabilityModifier>,

    orientation: Vec<i16>,
    _orientation: PhantomData<Orientation>,

    cover_image_path: Option<String>,
    _cover_image_path: PhantomData<CoverImagePath>,

    background_image_path: Option<String>,
    _background_image_path: PhantomData<BackgroundImagePath>,

    subscribers_quantity: i64,
    _subscribers_quantity: PhantomData<SubscribersQuantity>,

    marks_quantity: i64,
    _marks_quantity: PhantomData<MarksQuantity>,

    viewing_quantity: i64,
    _viewing_quantity: PhantomData<ViewingQuantity>,

    created_at: String,
    _created_at: PhantomData<CreatedAt>
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
        created_at: String
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
            _created_at: PhantomData
        };
    }

    pub fn into_inner(self) -> (
        i64,
        i64,
        Cow<'a, str>,
        String,
        Option<String>,
        i16,
        i16,
        Vec<i16>,
        Option<String>,
        Option<String>,
        i64,
        i64,
        i64,
        String
    ) {
        return (
            self.id,
            self.owner,
            self.name,
            self.linked_name,
            self.description,
            self.access_modifier,
            self.visability_modifier,
            self.orientation,
            self.cover_image_path,
            self.background_image_path,
            self.subscribers_quantity,
            self.marks_quantity,
            self.viewing_quantity,
            self.created_at
        );
    }

    pub fn get_id<'b>(&'b self) -> i64 {
        return self.id;
    }

    pub fn get_owner<'b>(&'b self) -> i64 {
        return self.owner;
    }

    pub fn get_access_modifier<'b>(&'b self) -> i16 {
        return self.access_modifier;
    }

    pub fn get_visability_modifier<'b>(&'b self) -> i16 {
        return self.visability_modifier;
    }
}