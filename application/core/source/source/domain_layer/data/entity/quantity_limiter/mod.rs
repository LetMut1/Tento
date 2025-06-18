mod field;
use {
    self::field::{
        CreatedAt,
        OwnedChannelsQuantity,
    },
    super::user::User_Id,
};
pub struct QuantityLimiter {
    user__id: User_Id,
    owned_channels_quantity: OwnedChannelsQuantity,
    created_at: CreatedAt,
}
pub type QuantityLimiter_OwnedChannelsQuantity = OwnedChannelsQuantity;
