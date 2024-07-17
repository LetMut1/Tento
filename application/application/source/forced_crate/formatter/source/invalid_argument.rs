use super::Formatter;
use aggregate_error::InvalidArgument;
impl Formatter<InvalidArgument> {
    pub fn format<'a>(invalid_argument: &'a InvalidArgument) -> String {
        return match *invalid_argument {
            InvalidArgument::FromOutside => "Invalid argument.".to_string(),
            InvalidArgument::FromClientCode {
                ref from_client_code
            } => format!(
                "Invalid argument: {}.",
                from_client_code.context.get(),
            ),
        };
    }
}
