use crate::domain_layer::data::entity::channel::Channel_VisabilityModifier_;

pub struct Channel_VisabilityModifierResolver;

impl Channel_VisabilityModifierResolver {
    const VISABILITY_MODIFIER_PUBLIC: i16 = 0;
    const VISABILITY_MODIFIER_PRIVATE: i16 = 1;

    pub fn from_representation(channel_visability_modifier: Channel_VisabilityModifier_) -> i16 {
        return match channel_visability_modifier {
            Channel_VisabilityModifier_::Public => Self::VISABILITY_MODIFIER_PUBLIC,
            Channel_VisabilityModifier_::Private => Self::VISABILITY_MODIFIER_PRIVATE
        };
    }

    pub fn to_representation(channel_visability_modifier: i16) -> Channel_VisabilityModifier_ {
        return match channel_visability_modifier {
            Self::VISABILITY_MODIFIER_PUBLIC => Channel_VisabilityModifier_::Public,
            Self::VISABILITY_MODIFIER_PRIVATE => Channel_VisabilityModifier_::Private,
            _ => Channel_VisabilityModifier_::Private
        };
    }
}