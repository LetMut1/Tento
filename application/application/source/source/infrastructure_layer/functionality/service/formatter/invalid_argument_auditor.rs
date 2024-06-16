use super::{
    Formatter,
    Formatter_,
};
use crate::infrastructure_layer::data::{
    auditor::{
        Auditor,
        Backtrace,
    },
    invalid_argument::InvalidArgument,
};
impl Formatter<Auditor<InvalidArgument>> {
    pub fn format<'a>(invalid_argument_auditor: &'a Auditor<InvalidArgument>) -> String {
        return format!(
            "Invalid argument.\n{}",
            Formatter_::<Backtrace>::format(&invalid_argument_auditor.backtrace).as_str(),
        );
    }
}
