use super::Formatter;
use aggregate_error::Logic;
impl Formatter<Logic> {
    pub fn format<'a>(logic: &'a Logic) -> String {
        return format!(
            "Logic: {}.",
            logic.message,
        );
    }
}
