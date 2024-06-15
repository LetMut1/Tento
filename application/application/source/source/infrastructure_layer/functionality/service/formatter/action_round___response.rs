use super::Formatter;
use crate::infrastructure_layer::data::control_type::ActionRound;
use crate::infrastructure_layer::data::control_type::Response;
impl Formatter<(ActionRound, Response)> {
    pub fn format<'a>(
        request_uri: &'a str,
        request_method: &'a str,
        response_status_code: u16,
    ) -> String {
        return format!(
            "\'{} {} {}\'",
            response_status_code, request_method, request_uri,
        );
    }
}
