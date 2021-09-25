pub struct OrderConventionResolver;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;

impl OrderConventionResolver {
    const ASC: &'static str = "ASC";
    const DESC: &'static str = "DESC";

    pub fn is_asc<'a>(
        order_in_integer_representation: &'a i8
    ) -> bool {
        return *order_in_integer_representation == 0;
    }

    pub fn is_desc<'a>(
        order_in_integer_representation: &'a i8
    ) -> bool {
        return *order_in_integer_representation == 1;
    }

    pub fn can_convert<'a>(
        order_in_integer_representation: &'a i8
    ) -> bool {
        if Self::is_asc(order_in_integer_representation) || Self::is_desc(order_in_integer_representation) {
            return true;
        }

        return false;
    }

    pub fn convert<'a>(
        order_in_integer_representation: &'a i8
    ) -> Result<&'static str, BaseError> {
        if Self::is_asc(order_in_integer_representation) {
            return Ok(Self::ASC);
        }
        if Self::is_desc(order_in_integer_representation) {
            return Ok(Self::DESC);
        }

        return Err(BaseError::InvalidArgumentError);
    }
}