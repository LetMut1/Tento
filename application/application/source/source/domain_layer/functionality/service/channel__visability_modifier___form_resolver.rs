use super::form_resolver::FormResolver;
use crate::domain_layer::data::entity::channel::Channel_VisabilityModifier;
use crate::domain_layer::data::entity::channel::Channel_VisabilityModifier_;

impl FormResolver<Channel_VisabilityModifier> {
    pub fn from_representation(channel_visability_modifier: Channel_VisabilityModifier_) -> Channel_VisabilityModifier {
        return match channel_visability_modifier {
            Channel_VisabilityModifier_::Public => Channel_VisabilityModifier(Channel_VisabilityModifier::PUBLIC),
            Channel_VisabilityModifier_::Private => Channel_VisabilityModifier(Channel_VisabilityModifier::PRIVATE),
        };
    }

    pub fn to_representation(channel_visability_modifier: Channel_VisabilityModifier) -> Channel_VisabilityModifier_ {
        return match channel_visability_modifier.0 {
            Channel_VisabilityModifier::PUBLIC => Channel_VisabilityModifier_::Public,
            _ => Channel_VisabilityModifier_::Private,
        };
    }
}
