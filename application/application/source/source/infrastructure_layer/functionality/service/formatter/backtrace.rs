use super::Formatter;
use super::Formatter_;
use crate::infrastructure_layer::data::auditor::Backtrace;

impl Formatter<Backtrace> {
    pub fn format<'a>(backtrace: &'a Backtrace) -> String {
        return Formatter_::<Backtrace>::format(backtrace);
    }
}
