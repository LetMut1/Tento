pub struct OrderConventionResolver;
use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;

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
    ) -> Result<&'static str, ErrorAggregator> {
        if Self::is_asc(order) {
            return Ok(Self::ASC);
        }
        if Self::is_desc(order) {
            return Ok(Self::DESC);
        }

        return Err(ErrorAggregator::InvalidArgumentError);
    }
}