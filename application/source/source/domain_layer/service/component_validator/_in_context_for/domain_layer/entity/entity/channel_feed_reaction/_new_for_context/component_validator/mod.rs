use crate::domain_layer::entity::entity::channel_feed_reaction::_component::value::Value;

pub struct ComponentValidator;

impl ComponentValidator {
    const VALUE_MAXIMUM_LENGTH: u16 = 300;

    pub fn is_valid_value<'outer_a>(value: &'outer_a Value) -> bool {
        return value.get_value().chars().count() <= (Self::VALUE_MAXIMUM_LENGTH as usize);
    }
}