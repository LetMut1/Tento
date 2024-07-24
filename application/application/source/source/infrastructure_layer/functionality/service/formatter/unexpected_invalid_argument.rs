use super::{
    Formatter,
    Formatter_,
    context_report,
};
use crate::infrastructure_layer::data::aggregate_error::InvalidArgument;
use crate::infrastructure_layer::data::server_workflow_error::UnexpectedInvalidArgument;
impl Formatter<UnexpectedInvalidArgument> {
    pub fn format<'a>(unexpected_invalid_argument: &'a UnexpectedInvalidArgument) -> String {
        return format!(
            context_report!(),
            Formatter_::<InvalidArgument>::INVALID_ARGUMET,
            unexpected_invalid_argument.from_client_code.context.get(),
        )
    }
}