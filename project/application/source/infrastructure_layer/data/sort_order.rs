use super::argument_result::ArgumentResult;
use super::argument_result::InvalidArgument;

pub enum SortOrder {
    Asc,
    Desc
}

impl SortOrder {
    const ASC_REPRESENTATION: i8 = 0;
    const DESC_REPRESENTATION: i8 = 1;
    const ASC: &'static str = "ASC";
    const DESC: &'static str = "DESC";

    pub fn new(sort_order_representation: i8) -> ArgumentResult<Self> {
        if sort_order_representation == Self::ASC_REPRESENTATION {
            return ArgumentResult::Ok { subject: SortOrder::Asc };
        }

        if sort_order_representation == Self::DESC_REPRESENTATION {
            return ArgumentResult::Ok { subject: SortOrder::Desc };
        }

        return ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::SortOrderRepresentation };
    }
}