use super::application_user::ApplicationUser_Id;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use std::borrow::Cow;

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

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct Id(i64);

impl Id {
    pub fn new(inner: i64) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i64 {
        return self.0;
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct Name(String);

impl Name {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct LinkedName(String);

impl LinkedName {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct Description(String);

impl Description {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Clone, Copy, Serialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct AccessModifier(i16);

impl AccessModifier {
    pub const OPEN: i16 = 0;
    pub const CLOSE: i16 = 1;

    pub fn new(inner: i16) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i16 {
        return self.0;
    }
}

pub enum AccessModifier_ {
    Open = AccessModifier::OPEN as isize,
    Close = AccessModifier::CLOSE as isize,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Clone, Copy, Serialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct VisabilityModifier(i16);

impl VisabilityModifier {
    pub const PUBLIC: i16 = 0;
    pub const PRIVATE: i16 = 1;

    pub fn new(inner: i16) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i16 {
        return self.0;
    }
}

pub enum VisabilityModifier_ {
    Public = VisabilityModifier::PUBLIC as isize,
    Private = VisabilityModifier::PRIVATE as isize,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct Orientation(Vec<i16>);

impl Orientation {
    pub fn new(inner: Vec<i16>) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a [i16] {
        return self.0.as_slice();
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct CoverImagePath(String);

impl CoverImagePath {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct BackgroundImagePath(String);

impl BackgroundImagePath {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Clone, Copy, Serialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct SubscribersQuantity(i64);

impl SubscribersQuantity {
    pub fn new(inner: i64) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i64 {
        return self.0;
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Clone, Copy, Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct MarksQuantity(i64);

impl MarksQuantity {
    pub fn new(inner: i64) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i64 {
        return self.0;
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Clone, Copy, Serialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
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
    pub id: Id,
    pub owner: ApplicationUser_Id,
    pub name: Cow<'a, Name>,
    pub linked_name: LinkedName,
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