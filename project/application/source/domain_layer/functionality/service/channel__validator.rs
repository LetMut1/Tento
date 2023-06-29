use super::validator::Validator;
use crate::domain_layer::data::entity::channel::Channel_Description;
use crate::domain_layer::data::entity::channel::Channel_Id;
use crate::domain_layer::data::entity::channel::Channel_LinkedName;
use crate::domain_layer::data::entity::channel::Channel_Name;
use crate::domain_layer::data::entity::channel::Channel_Orientation;

impl Validator<Channel_Id> {
    pub fn is_valid(channel_id: Channel_Id) -> bool {
        return channel_id.get() >= 0;
    }
}

impl Validator<Channel_Name> {
    pub const MAXIMUM_LENGTH: usize = 75;

    pub fn is_valid<'a>(channel_name: &'a Channel_Name) -> bool {
        let channel_name_ = channel_name.get();

        return channel_name_.chars().count() <= Self::MAXIMUM_LENGTH && !channel_name_.is_empty();
    }
}

impl Validator<Channel_LinkedName> {
    pub fn is_valid<'a>(channel_linked_name: &'a Channel_LinkedName) -> bool {
        return true; // TODO;
    }
}

impl Validator<Channel_Description> {
    pub const MAXIMUM_LENGTH: usize = 500;

    pub fn is_valid<'a>(channel_description: &'a Channel_Description) -> bool {
        return channel_description.get().chars().count() <= Self::MAXIMUM_LENGTH;
    }
}

impl Validator<Channel_Orientation> {
    pub fn is_valid<'a>(_channel_orientation: &'a Channel_Orientation) -> bool {
        return true; // TODO;
    }
}
