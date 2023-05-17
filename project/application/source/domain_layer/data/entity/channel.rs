use std::borrow::Cow;
use super::application_user::ApplicationUser_Id;

pub use self::Id as Channel_Id;
pub use self::Name as Channel_Name;
pub use self::LinkedName as Channel_LinkedName;
pub use self::Description as Channel_Description;
pub use self::AccessModifier_ as Channel_AccessModifier_;
pub use self::VisabilityModifier_ as Channel_VisabilityModifier_;
pub use self::Orientation as Channel_Orientation;
pub use self::CoverImagePath as Channel_CoverImagePath;
pub use self::BackgroundImagePath as Channel_BackgroundImagePath;
pub use self::SubscribersQuantity as Channel_SubscribersQuantity;
pub use self::MarksQuantity as Channel_MarksQuantity;
pub use self::ViewingQuantity as Channel_ViewingQuantity;
pub use self::CreatedAt as Channel_CreatedAt;

#[derive(Clone, Copy)]
pub struct Id(i64);

impl Id {
    pub fn new(inner: i64) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i64 {
        return self.0;
    }
}

#[derive(Clone)]
pub struct Name(String);

impl Name {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

pub struct LinkedName(String);

impl LinkedName {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

pub struct Description(String);

impl Description {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

#[derive(Clone, Copy)]
pub struct AccessModifier(i16);

impl AccessModifier {
    pub fn new(inner: i16) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i16 {
        return self.0;
    }
}

pub enum AccessModifier_ {
    /// 0 in integer representation.
    Open,
    /// 1 in integer representation.
    Close
}

#[derive(Clone, Copy)]
pub struct VisabilityModifier(i16);

impl VisabilityModifier {
    pub fn new(inner: i16) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i16 {
        return self.0;
    }
}

pub enum VisabilityModifier_ {
    /// 0 in integer representation.
    Public,
    /// 1 in integer representation.
    Private
}

pub struct Orientation(Vec<i16>);

impl Orientation {
    pub fn new(inner: Vec<i16>) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a [i16] {
        return self.0.as_slice();
    }
}

pub struct CoverImagePath(String);

impl CoverImagePath {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

pub struct BackgroundImagePath(String);

impl BackgroundImagePath {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

#[derive(Clone, Copy)]
pub struct SubscribersQuantity(i64);

impl SubscribersQuantity {
    pub fn new(inner: i64) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i64 {
        return self.0;
    }
}

#[derive(Clone, Copy)]
pub struct MarksQuantity(i64);

impl MarksQuantity {
    pub fn new(inner: i64) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i64 {
        return self.0;
    }
}

#[derive(Clone, Copy)]
pub struct ViewingQuantity(i64);

impl ViewingQuantity {
    pub fn new(inner: i64) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i64 {
        return self.0;
    }
}

pub struct CreatedAt(String);

impl CreatedAt {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

pub struct Channel<'a> {
    id: Id,
    owner: ApplicationUser_Id,
    name: Cow<'a, Name>,
    linked_name: LinkedName,
    description: Option<Description>,
    access_modifier: AccessModifier,
    visability_modifier: VisabilityModifier,
    orientation: Orientation,
    cover_image_path: Option<CoverImagePath>,
    background_image_path: Option<BackgroundImagePath>,
    subscribers_quantity: SubscribersQuantity,
    marks_quantity: MarksQuantity,
    viewing_quantity: ViewingQuantity,
    created_at: CreatedAt
}

impl<'a> Channel<'a> {
    pub fn new(
        id: Id,
        owner: ApplicationUser_Id,
        name: Cow<'a, Name>,
        linked_name: LinkedName,
        description: Option<Description>,
        access_modifier: AccessModifier,
        visability_modifier: VisabilityModifier,
        orientation: Orientation,
        cover_image_path: Option<CoverImagePath>,
        background_image_path: Option<BackgroundImagePath>,
        subscribers_quantity: SubscribersQuantity,
        marks_quantity: MarksQuantity,
        viewing_quantity: ViewingQuantity,
        created_at: CreatedAt
    ) -> Self {
        return Self {
            id,
            owner,
            name,
            linked_name,
            description,
            access_modifier,
            visability_modifier,
            orientation,
            cover_image_path,
            background_image_path,
            subscribers_quantity,
            marks_quantity,
            viewing_quantity,
            created_at
        };
    }

    pub fn into_inner(self) -> (
        Id,
        ApplicationUser_Id,
        Cow<'a, Name>,
        LinkedName,
        Option<Description>,
        AccessModifier,
        VisabilityModifier,
        Orientation,
        Option<CoverImagePath>,
        Option<BackgroundImagePath>,
        SubscribersQuantity,
        MarksQuantity,
        ViewingQuantity,
        CreatedAt
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

    pub fn get_id<'b>(&'b self) -> Id {
        return self.id;
    }

    pub fn get_owner<'b>(&'b self) -> ApplicationUser_Id {
        return self.owner;
    }

    pub fn get_access_modifier<'b>(&'b self) -> AccessModifier {
        return self.access_modifier;
    }

    pub fn get_visability_modifier<'b>(&'b self) -> VisabilityModifier {
        return self.visability_modifier;
    }
}