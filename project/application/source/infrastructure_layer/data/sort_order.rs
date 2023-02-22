use super::invalid_argument_result::InvalidArgument;
use super::invalid_argument_result::InvalidArgumentResult;

pub enum SortOrder {
    Asc,
    Desc
}

impl SortOrder {
    const ASC_REPRESENTATION: i8 = 0;
    const DESC_REPRESENTATION: i8 = 1;
    const ASC: &'static str = "ASC";
    const DESC: &'static str = "DESC";

    pub fn new(sort_order_representation: i8) -> InvalidArgumentResult<Self> {
        if sort_order_representation == Self::ASC_REPRESENTATION {
            return InvalidArgumentResult::Ok { subject: SortOrder::Asc };
        }

        if sort_order_representation == Self::DESC_REPRESENTATION {
            return InvalidArgumentResult::Ok { subject: SortOrder::Desc };
        }

        return InvalidArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::SortOrderRepresentation };
    }
}