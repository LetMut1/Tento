use super::Formatter;
use auditor::{
    Auditor,
};
use error::{
    Error,
    Internal,
    External,
};
impl Formatter<Error> {
    pub fn format<'a>(error: &'a Error) -> String {
        return match *error {
            Error::Internal { ref internal_auditor } => Formatter::<Auditor<Internal>>::format(internal_auditor),
            Error::External { ref external_auditor } => Formatter::<Auditor<External>>::format(external_auditor),
        };
    }
}
