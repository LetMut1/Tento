mod field;
use {
    self::field::{
        CreatedAt,
        Id,
        ImagesPathes,
        MarksQuantity,
        Text,
        view_quantity,
        CanBeDeletedFrom,
        IsPredeleted,
        ObfuscationValue,
    },
    super::channel::Channel_Id,
};
pub struct ChannelPublication1 {
    id: Id,
    channel__id: Channel_Id,
    images_pathes: ImagesPathes,
    text: Text,
    marks_quantity: MarksQuantity,
    view_quantity: view_quantity,
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
