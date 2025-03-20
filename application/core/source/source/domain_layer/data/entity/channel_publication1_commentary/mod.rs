mod field;
use {
    self::field::{
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
    user__id: User_Id,
    channel_publication1__id: ChannelPublication1_Id,
    text: Text,
    marks_quantity: MarksQuantity,
    created_at: CreatedAt,
}
