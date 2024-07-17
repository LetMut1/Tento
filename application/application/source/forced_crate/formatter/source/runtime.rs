use super::Formatter;
use aggregate_error::Runtime;
impl Formatter<Runtime> {
    pub fn format<'a>(runtime: &'a Runtime) -> String {
        return format!(
            "Runtime: {}.",
            runtime.context.get(),
        );
    }
}
