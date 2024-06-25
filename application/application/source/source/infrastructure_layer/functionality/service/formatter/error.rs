use super::{
    Formatter,
    Formatter_,
};
use crate::infrastructure_layer::data::{
    error::Error,
};
impl Formatter<Error> {
    pub fn format<'a>(error: &'a Error) -> String {
        return Formatter_::<Error>::format(error);
    }
}
