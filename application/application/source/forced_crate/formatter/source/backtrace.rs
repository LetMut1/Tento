use super::Formatter;
use aggregate_error::Backtrace;
impl Formatter<Backtrace> {
    pub fn format<'a>(backtrace: &'a Backtrace) -> String {
        return format!("{}:{}\n", backtrace.file_path, backtrace.line_number,);
    }
}
