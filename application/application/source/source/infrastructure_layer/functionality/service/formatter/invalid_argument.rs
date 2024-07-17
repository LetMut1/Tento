use super::{
    Formatter,
    Formatter_,
};
use crate::infrastructure_layer::data::aggregate_error::InvalidArgument;
impl Formatter<InvalidArgument> {
    pub fn format<'a>(invalid_argument: &'a InvalidArgument) -> String {
        return Formatter_::<InvalidArgument>::format(invalid_argument);
    }
}
