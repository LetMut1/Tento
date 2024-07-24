use super::{
    context_report,
    Formatter
};
use aggregate_error::Runtime;
impl Formatter<Runtime> {
    pub fn format<'a>(runtime: &'a Runtime) -> String {
        return format!(
            context_report!(),
            "Runtime",
            runtime.context.get(),
        );
    }
}
