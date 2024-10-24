use super::Formatter;
use crate::infrastructure_layer::data::aggregate_error::Backtrace;
impl Formatter<Backtrace> {
    pub fn format<'a>(backtrace: &'a Backtrace) -> String {
        return format!("{}:{}", backtrace.file_path, backtrace.line_number,);
    }
}
