mod field;
use {
    self::field::{
        CreatedAt,
        ImagesPathes,
        Text,
        Id,
        MarksQuantity,
        ViewingQuantity,
    },
    super::channel::Channel_Id,
};
pub struct ChannelPublication1 {
    id: Id,
    channel__id: Channel_Id,
    images_pathes: ImagesPathes,
    text: Text,
    marks_quantity: MarksQuantity,
    viewing_quantity: ViewingQuantity,
    created_at: CreatedAt,
}
pub type ChannelPublication1_Id = Id;