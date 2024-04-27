use super::Formatter;
use crate::infrastructure_layer::data::auditor::Backtrace;
use super::Formatter_;

impl Formatter<Backtrace> {
    pub fn format<'a>(backtrace: &'a Backtrace) -> String {
        return Formatter_::<Backtrace>::format(backtrace);
    }
}