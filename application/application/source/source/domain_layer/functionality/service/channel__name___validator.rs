use super::validator::Validator;
use crate::domain_layer::data::entity::channel::Channel_Name;

impl Validator<Channel_Name> {
    pub const MAXIMUM_LENGTH: usize = 75;

    pub fn is_valid<'a>(channel_name: &'a Channel_Name) -> bool {
        return channel_name.0.chars().count() <= Self::MAXIMUM_LENGTH && !channel_name.0.is_empty();
    }
}
