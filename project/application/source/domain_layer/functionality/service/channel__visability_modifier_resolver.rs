use crate::domain_layer::data::entity::channel::Channel_VisabilityModifier_;
use crate::domain_layer::data::entity::channel::Channel_VisabilityModifier;

pub struct Channel_VisabilityModifierResolver;

impl Channel_VisabilityModifierResolver {
    pub fn from_representation(channel_visability_modifier: Channel_VisabilityModifier_) -> Channel_VisabilityModifier {
        return match channel_visability_modifier {
            Channel_VisabilityModifier_::Public => Channel_VisabilityModifier::new(Channel_VisabilityModifier::PUBLIC),
            Channel_VisabilityModifier_::Private => Channel_VisabilityModifier::new(Channel_VisabilityModifier::PRIVATE)
        };
    }

    pub fn to_representation(channel_visability_modifier: Channel_VisabilityModifier) -> Channel_VisabilityModifier_ {
        return match channel_visability_modifier.get() {
            Channel_VisabilityModifier::PUBLIC => Channel_VisabilityModifier_::Public,
            Channel_VisabilityModifier::PRIVATE => Channel_VisabilityModifier_::Private,
            _ => Channel_VisabilityModifier_::Private
        };
    }
}