mod field;
use {
    self::field::{
        Id,
        CreatedAt,
        Text,
        MarksQuantity,
    },
    super::{
        channel_publication1::ChannelPublication1_Id,
        user::User_Id,
    },
};
pub struct ChannelPublication1Commentary {
    id: Id,
    user__id: User_Id,
    channel_publication1__id: ChannelPublication1_Id,
    text: Text,
    marks_quantity: MarksQuantity,
    created_at: CreatedAt,
}
pub type ChannelPublication1Commentary_Text = Text;
