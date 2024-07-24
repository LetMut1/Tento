use super::{
    context_report,
    Formatter
};
use aggregate_error::Logic;
impl Formatter<Logic> {
    pub fn format<'a>(logic: &'a Logic) -> String {
        return format!(
            context_report!(),
            "Logic",
            logic.message,
        );
    }
}
