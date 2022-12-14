use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;

pub struct IntegerTypeConverter;

impl IntegerTypeConverter {
    pub fn convert_i16_to_u8(
        value: i16
    ) -> Result<u8, ErrorAuditor> {
        if value >= (u8::MIN as i16) && value <= (u8::MAX as i16) {
            return Ok(value as u8);
        } else {
            return Err(
                ErrorAuditor::new(
                    BaseError::LogicError { logic_error: LogicError::new(false, "Can not convert value without changing.") },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }
    }
}