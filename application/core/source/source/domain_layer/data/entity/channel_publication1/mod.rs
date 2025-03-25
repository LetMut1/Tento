mod field;
use {
    self::field::{
        CanBeDeletedFrom,
        CommentariesQuantity,
        CreatedAt,
        Id,
        ImagesPathes,
        IsPredeleted,
        MarksQuantity,
        ObfuscationValue,
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
    obfuscation_value: ObfuscationValue,
    created_at: CreatedAt,
    is_predeleted: IsPredeleted,
    can_be_deleted_from: CanBeDeletedFrom,
}
pub type ChannelPublication1_Id = Id;
pub type ChannelPublication1_ImagesPathes = ImagesPathes;
pub type ChannelPublication1_Text = Text;
pub type ChannelPublication1_CanBeDeletedFrom = CanBeDeletedFrom;
pub type ChannelPublication1_ObfuscationValue = ObfuscationValue;
