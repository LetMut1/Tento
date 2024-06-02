use super::FormResolver;
use crate::domain_layer::data::entity::channel::Channel_AccessModifier;
use crate::domain_layer::data::entity::channel::Channel_AccessModifier_;

impl FormResolver<Channel_AccessModifier> {
    pub fn from_representation(channel_access_modifier: Channel_AccessModifier_) -> i16 {
        return match channel_access_modifier {
            Channel_AccessModifier_::Open => Channel_AccessModifier::OPEN,
            Channel_AccessModifier_::Close => Channel_AccessModifier::CLOSE,
        };
    }

    pub fn to_representation(channel_access_modifier: i16) -> Channel_AccessModifier_ {
        return if channel_access_modifier == Channel_AccessModifier::OPEN {
            Channel_AccessModifier_::Open
        } else {
            Channel_AccessModifier_::Close
        };
    }
}
