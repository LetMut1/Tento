use super::FormResolver;
use crate::domain_layer::data::entity::channel::Channel_AccessModifier;
use crate::domain_layer::data::entity::channel::Channel_AccessModifier_;

impl FormResolver<Channel_AccessModifier> {
    pub fn from_representation(channel_access_modifier: Channel_AccessModifier_) -> Channel_AccessModifier {
        return match channel_access_modifier {
            Channel_AccessModifier_::Open => Channel_AccessModifier(Channel_AccessModifier::OPEN),
            Channel_AccessModifier_::Close => Channel_AccessModifier(Channel_AccessModifier::CLOSE),
        };
    }

    pub fn to_representation(channel_access_modifier: Channel_AccessModifier) -> Channel_AccessModifier_ {
        return match channel_access_modifier.0 {
            Channel_AccessModifier::OPEN => Channel_AccessModifier_::Open,
            _ => Channel_AccessModifier_::Close,
        };
    }
}
