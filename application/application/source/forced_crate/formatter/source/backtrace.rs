use super::Formatter;
use auditor::Backtrace;

impl Formatter<Backtrace> {
    pub fn format<'a>(backtrace: &'a Backtrace) -> String {
        return format!(
            "{}:{}.\n",
            backtrace.file_path,
            backtrace.line_number,
        );
    }
}