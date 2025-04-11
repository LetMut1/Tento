mod field;
use {
    self::field::{
        CanBeDeletedFrom,
        CreatedAt,
    },
    super::channel_publication1_commentary::ChannelPublication1Commentary_Id,
};
pub struct ChannelPublication1CommentaryDelayedDeletion {
    channel_publication1_commentary__id: ChannelPublication1Commentary_Id,
    can_be_deleted_from: CanBeDeletedFrom,
    created_at: CreatedAt,
}
pub type ChannelPublication1CommentaryDelayedDeletion_CanBeDeletedFrom = CanBeDeletedFrom;
