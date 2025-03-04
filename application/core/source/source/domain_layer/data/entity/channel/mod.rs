mod field;
use {
    self::field::{
        AccessModifier_,
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
        VisabilityModifier_,
        ObfuscationValue,
        AccessModifier,
        VisabilityModifier,
    },
    super::user::User_Id,
};
pub struct Channel {
    id: Id,
    owner: User_Id,
    name: Name,
    linked_name: LinkedName,
    description: Description,
    access_modifier: AccessModifier,
    visability_modifier: VisabilityModifier,
    orientation: Orientation,
    cover_image_path: CoverImagePath,
    background_image_path: BackgroundImagePath,
    subscribers_quantity: SubscribersQuantity,
    marks_quantity: MarksQuantity,
    viewing_quantity: ViewingQuantity,
    obfuscation_value: ObfuscationValue,
    created_at: CreatedAt,
}
pub type Channel_BackgroundImagePath = BackgroundImagePath;
pub type Channel_CoverImagePath = CoverImagePath;
pub type Channel_CreatedAt = CreatedAt;
pub type Channel_MarksQuantity = MarksQuantity;
pub type Channel_Orientation = Orientation;
pub type Channel_SubscribersQuantity = SubscribersQuantity;
pub type Channel_ViewingQuantity = ViewingQuantity;
pub type Channel_AccessModifier_ = AccessModifier_;
pub type Channel_AccessModifier = AccessModifier;
pub type Channel_Description = Description;
pub type Channel_Id = Id;
pub type Channel_LinkedName = LinkedName;
pub type Channel_Name = Name;
pub type Channel_VisabilityModifier_ = VisabilityModifier_;
pub type Channel_VisabilityModifier = VisabilityModifier;
pub type Channel_ObfuscationValue = ObfuscationValue;