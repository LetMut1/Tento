mod field;
use {
    self::field::{
        AccessModifier,
        BackgroundImagePath,
        CoverImagePath,
        CreatedAt,
        Description,
        Id,
        LinkedName,
        MarksQuantity,
        Name,
        Orientation,
        SubscribersQuantity,
        ViewingQuantity,
        VisabilityModifier,
        ObfuscationValue,
    },
    super::user::User_Id,
    std::marker::PhantomData,
};
pub struct Channel {
    id: i64,
    _id: PhantomData<Id>,
    owner: i64,
    _owner: PhantomData<User_Id>,
    name: String,
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
    obfuscation_value: i64,
    _obfuscation_value: PhantomData<ObfuscationValue>,
    created_at: i64,
    _created_at: PhantomData<CreatedAt>,
}
pub type Channel_BackgroundImagePath = BackgroundImagePath;
pub type Channel_CoverImagePath = CoverImagePath;
pub type Channel_CreatedAt = CreatedAt;
pub type Channel_MarksQuantity = MarksQuantity;
pub type Channel_Orientation = Orientation;
pub type Channel_SubscribersQuantity = SubscribersQuantity;
pub type Channel_ViewingQuantity = ViewingQuantity;
pub type Channel_AccessModifier = AccessModifier;
pub type Channel_Description = Description;
pub type Channel_Id = Id;
pub type Channel_LinkedName = LinkedName;
pub type Channel_Name = Name;
pub type Channel_VisabilityModifier = VisabilityModifier;
pub type Channel_ObfuscationValue = ObfuscationValue;