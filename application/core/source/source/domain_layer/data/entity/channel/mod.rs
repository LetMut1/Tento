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
    id: PhantomData<(i64, Id)>,
    owner: PhantomData<(i64, User_Id)>,
    name: PhantomData<(String, Name)>,
    linked_name: PhantomData<(String, LinkedName)>,
    description: PhantomData<(Option<String>, Description)>,
    access_modifier: PhantomData<(i16, AccessModifier)>,
    visability_modifier: PhantomData<(i16, VisabilityModifier)>,
    orientation: PhantomData<(Vec<i16>, Orientation)>,
    cover_image_path: PhantomData<(Option<String>, CoverImagePath)>,
    background_image_path: PhantomData<(Option<String>, BackgroundImagePath)>,
    subscribers_quantity: PhantomData<(i64, SubscribersQuantity)>,
    marks_quantity: PhantomData<(i64, MarksQuantity)>,
    viewing_quantity: PhantomData<(i64, ViewingQuantity)>,
    obfuscation_value: PhantomData<(i64, ObfuscationValue)>,
    created_at: PhantomData<(i64, CreatedAt)>,
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