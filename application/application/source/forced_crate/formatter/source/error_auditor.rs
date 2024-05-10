use error::Error;
use auditor::Auditor;
use auditor::Backtrace;
use error::Runtime;
use super::Formatter;

impl Formatter<Auditor<Error>> {
    pub fn format<'a>(error_auditor: &'a Auditor<Error>) -> String {
        todo!();
        // let backtrace_message = Formatter::<Backtrace>::format(error_auditor.get_backtrace());

        // let error_message = match *error_auditor.get_subject() {
        //     Error::Logic {
        //         message,
        //     } => {
        //         format!(
        //             "LogicError: {}.",
        //             message
        //         )
        //     }
        //     Error::Runtime {
        //         runtime: ref run_time_error,
        //     } => {
        //         match *run_time_error {
        //             Runtime::Other {
        //                 ref other,
        //             } => {
        //                 format!(
        //                     "OtherRuntimeError: {}.",
        //                     other.get_error()
        //                 )
        //             }
        //             Runtime::Resource {
        //                 ref resource,
        //             } => {
        //                 match *resource {
        //                     Resource::Clickhouse => "ClickhouseResourceRuntimeError.".to_string(),
        //                 }
        //             }
        //         }
        //     },
        // };


        // return format!(
        //     "{}:\n{}",
        //     error_message.as_str(),
        //     backtrace_message.as_str(),
        // );
    }
}