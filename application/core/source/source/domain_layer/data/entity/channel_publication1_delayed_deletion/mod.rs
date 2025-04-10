mod field;
use {
    self::field::{
        CanBeDeletedFrom,
        CreatedAt,
    },
    super::channel_publication1::ChannelPublication1_Id,
};
pub struct ChannelPublication1DelayedDeletion {
    channel_publication1__id: ChannelPublication1_Id,
    can_be_deleted_from: CanBeDeletedFrom,
    created_at: CreatedAt,
}
pub type ChannelPublication1DelayedDeletion_CanBeDeletedFrom = CanBeDeletedFrom;
