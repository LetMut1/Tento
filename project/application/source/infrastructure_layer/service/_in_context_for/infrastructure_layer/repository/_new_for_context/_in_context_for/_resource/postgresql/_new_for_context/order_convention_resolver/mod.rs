pub struct OrderConventionResolver;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;

impl OrderConventionResolver {
    const ASC: &'static str = "ASC";
    const DESC: &'static str = "DESC";

    pub fn is_asc<'a>(
        order: &'a i8
    ) -> bool {
        return *order == 0;
    }

    pub fn is_desc<'a>(
        order: &'a i8
    ) -> bool {
        return *order == 1;
    }

    pub fn can_convert<'a>(
        order: &'a i8
    ) -> bool {
        if Self::is_asc(order) || Self::is_desc(order) {
            return true;
        }

        return false;
    }

    pub fn convert<'a>(
        order: &'a i8
    ) -> Result<&'static str, ErrorAuditor> {
        if Self::is_asc(order) {
            return Ok(Self::ASC);
        }
        if Self::is_desc(order) {
            return Ok(Self::DESC);
        }

        return Err(
            ErrorAuditor::new(
                ErrorAggregator::InvalidArgumentError,
                BacktracePart::new(line!(), file!(), None)
            )
        );
    }
}