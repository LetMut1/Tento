mod field;
use {
    self::field::{
        CreatedAt,
        ImagesPathes,
        Text,
        Id,
    },
    super::channel::Channel_Id,
};
pub struct ChannelPublication1 {
    id: Id,
    channel__id: Channel_Id,
    images_pathes: ImagesPathes,
    text: Text,
    created_at: CreatedAt,
}