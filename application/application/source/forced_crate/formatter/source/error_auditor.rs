use error::Error;
use auditor::Auditor;
use super::Formatter;

impl Formatter<Auditor<Error>> {
    pub fn format<'a>(error_auditor: &'a Auditor<Error>) -> String {
        todo!();
        // return Formatter_::prepare(error_auditor);
    }
}