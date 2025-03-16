mod field;
use {
    self::field::{
        AccessModifier,
        AccessModifier_,
        BackgroundImagePath,
        CoverImagePath,
        CreatedAt,
        Description,
        Id,
        LinkedName,
        Name,
        ObfuscationValue,
        Orientation,
        SubscribersQuantity,
        VisabilityModifier,
        VisabilityModifier_,
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
    obfuscation_value: ObfuscationValue,
    created_at: CreatedAt,
}
pub type Channel_Orientation = Orientation;
pub type Channel_AccessModifier_ = AccessModifier_;
pub type Channel_Description = Description;
pub type Channel_Id = Id;
pub type Channel_LinkedName = LinkedName;
pub type Channel_Name = Name;
pub type Channel_VisabilityModifier_ = VisabilityModifier_;
pub type Channel_ObfuscationValue = ObfuscationValue;
