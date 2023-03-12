use crate::domain_layer::data::entity::channel::AccessModifier;

pub struct Channel_AccessModifierResolver;

impl Channel_AccessModifierResolver {
    const ACCESS_MODIFIER_OPEN: i16 = 0;
    const ACCESS_MODIFIER_CLOSE: i16 = 1;

    pub fn from_representation(channel_access_modifier: AccessModifier) -> i16 {
        return match channel_access_modifier {
            AccessModifier::Open => Self::ACCESS_MODIFIER_OPEN,
            AccessModifier::Close => Self::ACCESS_MODIFIER_CLOSE
        };
    }

    pub fn to_representation(channel_access_modifier: i16) -> AccessModifier {
        return match channel_access_modifier {
            Self::ACCESS_MODIFIER_OPEN => AccessModifier::Open,
            Self::ACCESS_MODIFIER_CLOSE => AccessModifier::Close,
            _ => AccessModifier::Close
        };
    }
}