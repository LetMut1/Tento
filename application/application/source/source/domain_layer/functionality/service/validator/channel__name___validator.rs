use super::Validator;
use crate::domain_layer::data::entity::channel::Channel_Name;

impl Validator<Channel_Name> {
    pub fn is_valid<'a>(channel_name: &'a Channel_Name) -> bool {
        return channel_name.0.chars().count() <= Channel_Name::MAXIMUM_LENGTH && !channel_name.0.is_empty();
    }
}
