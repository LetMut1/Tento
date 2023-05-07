use crate::domain_layer::data::entity::channel::Channel_AccessModifier_;

pub struct Channel_AccessModifierResolver;

impl Channel_AccessModifierResolver {
    const ACCESS_MODIFIER_OPEN: i16 = 0;
    const ACCESS_MODIFIER_CLOSE: i16 = 1;

    pub fn from_representation(channel_access_modifier: Channel_AccessModifier_) -> i16 {
        return match channel_access_modifier {
            Channel_AccessModifier_::Open => Self::ACCESS_MODIFIER_OPEN,
            Channel_AccessModifier_::Close => Self::ACCESS_MODIFIER_CLOSE
        };
    }

    pub fn to_representation(channel_access_modifier: i16) -> Channel_AccessModifier_ {
        return match channel_access_modifier {
            Self::ACCESS_MODIFIER_OPEN => Channel_AccessModifier_::Open,
            Self::ACCESS_MODIFIER_CLOSE => Channel_AccessModifier_::Close,
            _ => Channel_AccessModifier_::Close
        };
    }
}