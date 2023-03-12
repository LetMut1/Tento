use crate::domain_layer::data::entity::channel::VisabilityModifier;

pub struct Channel_VisabilityModifierResolver;

impl Channel_VisabilityModifierResolver {
    const VISABILITY_MODIFIER_PUBLIC: i16 = 0;
    const VISABILITY_MODIFIER_PRIVATE: i16 = 1;

    pub fn from_representation(channel_visability_modifier: VisabilityModifier) -> i16 {
        return match channel_visability_modifier {
            VisabilityModifier::Public => Self::VISABILITY_MODIFIER_PUBLIC,
            VisabilityModifier::Private => Self::VISABILITY_MODIFIER_PRIVATE
        };
    }

    pub fn to_representation(channel_visability_modifier: i16) -> VisabilityModifier {
        return match channel_visability_modifier {
            Self::VISABILITY_MODIFIER_PUBLIC => VisabilityModifier::Public,
            Self::VISABILITY_MODIFIER_PRIVATE => VisabilityModifier::Private,
            _ => VisabilityModifier::Private
        };
    }
}