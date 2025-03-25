mod field;
use {
    self::field::{
        CanBeDeletedFrom,
        CreatedAt,
        Id,
        IsPredeleted,
        MarksQuantity,
        Text,
    },
    super::{
        channel_publication1::ChannelPublication1_Id,
        user::User_Id,
    },
};
pub struct ChannelPublication1Commentary {
    id: Id,
    author: User_Id,
    channel_publication1__id: ChannelPublication1_Id,
    text: Text,
    marks_quantity: MarksQuantity,
    created_at: CreatedAt,
    is_predeleted: IsPredeleted,
    can_be_deleted_from: CanBeDeletedFrom,
}
pub type ChannelPublication1Commentary_Id = Id;
pub type ChannelPublication1Commentary_Text = Text;
pub type ChannelPublication1Commentary_CanBeDeletedFrom = CanBeDeletedFrom;
