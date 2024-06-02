use super::FormResolver;
use crate::domain_layer::data::entity::channel::Channel_VisabilityModifier;
use crate::domain_layer::data::entity::channel::Channel_VisabilityModifier_;

impl FormResolver<Channel_VisabilityModifier> {
    pub fn from_representation(channel_visability_modifier: Channel_VisabilityModifier_) -> i16 {
        return match channel_visability_modifier {
            Channel_VisabilityModifier_::Public => Channel_VisabilityModifier::PUBLIC,
            Channel_VisabilityModifier_::Private => Channel_VisabilityModifier::PRIVATE,
        };
    }

    pub fn to_representation(channel_visability_modifier: i16) -> Channel_VisabilityModifier_ {
        return if channel_visability_modifier == Channel_VisabilityModifier::PUBLIC {
            Channel_VisabilityModifier_::Public
        } else {
            Channel_VisabilityModifier_::Private
        };
    }
}
