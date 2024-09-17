use super::Formatter;
use aggregate_error::InvalidArgument;
use std::marker::PhantomData;
impl Formatter<PhantomData<InvalidArgument>> {
    pub fn format() -> String {
        return "Invalid argument".to_string();
    }
}
