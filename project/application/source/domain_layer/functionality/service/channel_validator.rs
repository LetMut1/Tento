use crate::domain_layer::data::entity::channel::Channel_Description;
use crate::domain_layer::data::entity::channel::Channel_Id;
use crate::domain_layer::data::entity::channel::Channel_LinkedName;
use crate::domain_layer::data::entity::channel::Channel_Name;
use crate::domain_layer::data::entity::channel::Channel_Orientation;
use super::validator::Validator;

impl Validator<Channel_Id> {
    pub fn is_valid<'a>(channel_id: i64) -> bool {
        return channel_id >= 0;
    }
}

impl Validator<Channel_Name> {
    pub const MAXIMUM_LENGTH: usize = 75;

    pub fn is_valid<'a>(channel_name: &'a str) -> bool {
        return channel_name.chars().count() <= Self::MAXIMUM_LENGTH
            && !channel_name.is_empty();
    }
}

impl Validator<Channel_LinkedName> {
    pub fn is_valid<'a>(channel_linked_name: &'a str) -> bool {
        return true;    // TODO;
    }
}

impl Validator<Channel_Description> {
    pub const MAXIMUM_LENGTH: usize = 500;

    pub fn is_valid<'a>(channel_description: &'a str) -> bool {
        return channel_description.chars().count() <= Self::MAXIMUM_LENGTH;
    }
}

impl Validator<Channel_Orientation> {
    pub fn is_valid<'a>(_channel_orientation: &'a [i16]) -> bool {
        return true;    // TODO;
    }
}