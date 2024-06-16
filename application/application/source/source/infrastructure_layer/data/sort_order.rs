pub enum SortOrder {
    Asc,
    Desc,
}
impl SortOrder {
    const ASC: &'static str = "ASC";
    const ASC_REPRESENTATION: i8 = 0;
    const DESC: &'static str = "DESC";
    const DESC_REPRESENTATION: i8 = 1;
    pub fn new(sort_order_representation: i8) -> Self {
        return match sort_order_representation {
            Self::DESC_REPRESENTATION => SortOrder::Desc,
            _ => SortOrder::Asc,
        };
    }
    pub fn convert(self) -> &'static str {
        return match self {
            Self::Asc => Self::ASC,
            Self::Desc => Self::DESC,
        };
    }
}
