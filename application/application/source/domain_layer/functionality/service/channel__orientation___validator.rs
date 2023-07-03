use super::validator::Validator;
use crate::domain_layer::data::entity::channel::Channel_Orientation;

impl Validator<Channel_Orientation> {
    pub fn is_valid<'a>(_channel_orientation: &'a Channel_Orientation) -> bool {
        return true; // TODO;
    }
}
