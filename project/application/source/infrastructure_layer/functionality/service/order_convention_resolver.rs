pub struct OrderConventionResolver;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;

impl OrderConventionResolver {
    const ASC_REPRESENTATION: i8 = 0;
    const DESC_REPRESENTATION: i8 = 1;
    const ASC: &'static str = "ASC";
    const DESC: &'static str = "DESC";

    pub fn is_asc(order: i8) -> bool {
        return order == Self::ASC_REPRESENTATION;
    }

    pub fn is_desc(order: i8) -> bool {
        return order == Self::DESC_REPRESENTATION;
    }

    pub fn can_convert(order:  i8) -> bool {
        if Self::is_asc(order) || Self::is_desc(order) {
            return true;
        }

        return false;
    }

    pub fn convert(order: i8) -> Result<&'static str, ErrorAuditor> {
        if Self::is_asc(order) {
            return Ok(Self::ASC);
        }
        if Self::is_desc(order) {
            return Ok(Self::DESC);
        }





        todo!();
    }
}