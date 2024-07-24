use super::{
    Formatter,
    Formatter_,
    context_report,
};
use crate::infrastructure_layer::data::aggregate_error::InvalidArgument;
use crate::infrastructure_layer::data::server_workflow_error::ExpectedInvalidArgument;
impl Formatter<ExpectedInvalidArgument> {
    pub fn format<'a>(expected_invalid_argument: &'a ExpectedInvalidArgument) -> String {
        return match *expected_invalid_argument {
            ExpectedInvalidArgument::FromOutside => Formatter_::<InvalidArgument>::INVALID_ARGUMET.to_string(),
            ExpectedInvalidArgument::FromOutsideAndClientCode {
                ref from_outside_and_client_code
            } => format!(
                context_report!(),
                Formatter_::<InvalidArgument>::INVALID_ARGUMET,
                from_outside_and_client_code.context.get(),
            )
        };
    }
}