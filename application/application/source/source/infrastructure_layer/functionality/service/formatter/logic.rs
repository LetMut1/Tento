use super::{
    Formatter,
    Formatter_,
};
use crate::infrastructure_layer::data::aggregate_error::Logic;
impl Formatter<Logic> {
    pub fn format<'a>(logic: &'a Logic) -> String {
        return Formatter_::<Logic>::format(logic);
    }
}
