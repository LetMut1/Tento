use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use extern_crate::hyper::Body;
use extern_crate::hyper::Response;
use super::action_response_creator::ActionResponseCreator;

pub struct ActionUnexpectedResponseCreator;

impl ActionUnexpectedResponseCreator {
    pub fn create<'a>(error_auditor: &'a ErrorAuditor) -> Response<Body> {
        match *error_auditor.get_base_error() {
            BaseError::InvalidArgumentError => {
                return ActionResponseCreator::create_bad_request();
            }
            BaseError::LogicError { logic_error: _ } |
            BaseError::RunTimeError { run_time_error: _ } => {
                // TODO log::error!("{}", error);

                return ActionResponseCreator::create_internal_server_error();
            }
        }
    }
}