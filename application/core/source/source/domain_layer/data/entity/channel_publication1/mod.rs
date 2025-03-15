mod field;
use {
    self::field::{
        CreatedAt,
        Id,
        ImagesPathes,
        MarksQuantity,
        Text,
        ViewingQuantity,
        CanBeDeletedFrom,
        IsPredeleted,
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
    is_predeleted: IsPredeleted,
    can_be_deleted_from: CanBeDeletedFrom,
}
pub type ChannelPublication1_Id = Id;
pub type ChannelPublication1_ImagesPathes = ImagesPathes;
pub type ChannelPublication1_Text = Text;
pub type ChannelPublication1_CanBeDeletedFrom = CanBeDeletedFrom;
