use super::Formatter;
use crate::infrastructure_layer::data::aggregate_error::InvalidArgument;
use std::marker::PhantomData;
impl Formatter<PhantomData<InvalidArgument>> {
    pub fn format() -> String {
        return "Invalid argument".to_string();
    }
}
