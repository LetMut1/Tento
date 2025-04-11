mod field;
use {
    self::field::{
        CanBeDeletedFrom,
        CreatedAt,
    },
    super::channel::Channel_Id,
};
pub struct ChannelDelayedDeletion {
    channel__id: Channel_Id,
    can_be_deleted_from: CanBeDeletedFrom,
    created_at: CreatedAt,
}
pub type ChannelDelayedDeletion_CanBeDeletedFrom = CanBeDeletedFrom;
