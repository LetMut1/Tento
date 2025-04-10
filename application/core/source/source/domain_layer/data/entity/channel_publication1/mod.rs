mod field;
use {
    self::field::{
        CommentariesQuantity,
        CreatedAt,
        Id,
        ImagesPathes,
        MarksQuantity,
        Text,
        ViewQuantity,
    },
    super::channel::Channel_Id,
};
pub struct ChannelPublication1 {
    id: Id,
    channel__id: Channel_Id,
    images_pathes: ImagesPathes,
    text: Text,
    commentaries_quantity: CommentariesQuantity,
    marks_quantity: MarksQuantity,
    view_quantity: ViewQuantity,
    created_at: CreatedAt,
}
pub type ChannelPublication1_Id = Id;
pub type ChannelPublication1_ImagesPathes = ImagesPathes;
pub type ChannelPublication1_Text = Text;
