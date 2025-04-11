mod field;
use {
    self::field::{
        CreatedAt,
        Id,
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
}
pub type ChannelPublication1Commentary_Id = Id;
pub type ChannelPublication1Commentary_Text = Text;
