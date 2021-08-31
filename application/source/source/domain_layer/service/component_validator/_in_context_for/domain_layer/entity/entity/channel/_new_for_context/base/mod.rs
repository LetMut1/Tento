use crate::domain_layer::entity::entity::channel::_component::description::Description;
use crate::domain_layer::entity::entity::channel::_component::name::Name;

pub struct Base;

impl Base {
    const NAME_MAXIMUM_LENGTH: u8 = 75;
    const DESCRIPTION_MAXIMUM_LENGTH: u16 = 500;

    pub fn is_valid_name<'outer_a>(name: &'outer_a Name) -> bool {
        return name.get_value().chars().count() <= (Self::NAME_MAXIMUM_LENGTH as usize);
    }
    
    pub fn is_valid_description<'outer_a>(description: &'outer_a Description) -> bool {
        return description.get_value().chars().count() <= (Self::DESCRIPTION_MAXIMUM_LENGTH as usize);
    }
}