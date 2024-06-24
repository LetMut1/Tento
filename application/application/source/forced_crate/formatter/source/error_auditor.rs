use super::Formatter;
use auditor::{
    Auditor,
    Backtrace,
};
use error::{
    Error,
    Internal,
};
impl Formatter<Auditor<Error>> {
    pub fn format<'a>(error_auditor: &'a Auditor<Error>) -> String {
        let error_message = match error_auditor.subject {
            Error::Internal { ref internal } => {
                let error_message_ = match *internal {
                    Internal::Logic {
                        message,
                    } => format!("Logic: {}.", message),
                    Internal::Runtime {
                        ref runtime,
                    } => format!("Runtime: {}.", runtime.get(),),
                };
                error_message_
            }
        };
        return format!(
            "{}:\n{}",
            error_message.as_str(),
            Formatter::<Backtrace>::format(&error_auditor.backtrace).as_str(),
        );
    }
}
