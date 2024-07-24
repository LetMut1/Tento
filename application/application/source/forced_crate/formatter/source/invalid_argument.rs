use super::{
    context_report,
    Formatter
};
use aggregate_error::InvalidArgument;
impl Formatter<InvalidArgument> {
    pub const INVALID_ARGUMET: &'static str = "Invalid argument";
    pub fn format<'a>(invalid_argument: &'a InvalidArgument) -> String {
        return match *invalid_argument {
            InvalidArgument::FromOutside => Self::INVALID_ARGUMET.to_string(),
            InvalidArgument::FromClientCode {
                ref from_client_code,
            } => {
                format!(
                    context_report!(),
                    Self::INVALID_ARGUMET,
                    from_client_code.context.get(),
                )
            }
            InvalidArgument::FromOutsideAndClientCode {
                ref from_outside_and_client_code
            } => {
                format!(
                    context_report!(),
                    Self::INVALID_ARGUMET,
                    from_outside_and_client_code.context.get(),
                )
            }
        };
    }
}
