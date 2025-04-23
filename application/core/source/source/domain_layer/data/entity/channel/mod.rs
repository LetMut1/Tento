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
    description: Option<Description>,
    access_modifier: AccessModifier,
    visability_modifier: VisabilityModifier,
    cover_image_path: Option<CoverImagePath>,
    background_image_path: Option<BackgroundImagePath>,
    subscribers_quantity: SubscribersQuantity,
    created_at: CreatedAt,
}
pub type Channel_AccessModifier_ = AccessModifier_;
pub type Channel_Description = Description;
pub type Channel_Id = Id;
pub type Channel_LinkedName = LinkedName;
pub type Channel_Name = Name;
pub type Channel_VisabilityModifier_ = VisabilityModifier_;
