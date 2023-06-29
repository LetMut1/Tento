use super::validator::Validator;
use crate::domain_layer::data::entity::channel::Channel_Description;

impl Validator<Channel_Description> {
    pub const MAXIMUM_LENGTH: usize = 500;

    pub fn is_valid<'a>(channel_description: &'a Channel_Description) -> bool {
        return channel_description.get().chars().count() <= Self::MAXIMUM_LENGTH;
    }
}