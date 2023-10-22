use super::Formatter;

pub use crate::infrastructure_layer::data::control_type::ActionRoundLog;

impl Formatter<ActionRoundLog> {
    pub fn format<'a>(
        request_uri: &'a str,
        request_method: &'a str,
        response_status_code: u16,
        context: Option<&'a str>
    ) -> String {
        let message = match context {
            Some(context_) => format!(
                "{{ {} {} {} }} {}",
                request_uri,
                request_method,
                response_status_code,
                context_,
            ),
            None => format!(
                "{{ {} {} {} }}",
                request_uri,
                request_method,
                response_status_code,
            )
        };

        return message;
    }
}