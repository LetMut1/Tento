use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::LogicError;

pub struct IntegerTypeConverter;

impl IntegerTypeConverter {
    pub fn convert_i16_to_u8(value: i16) -> Result<u8, ErrorAuditor> {
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