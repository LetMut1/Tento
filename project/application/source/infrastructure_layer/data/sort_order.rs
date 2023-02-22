use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;

pub enum SortOrder {
    Asc,
    Desc
}

impl SortOrder {
    const ASC_REPRESENTATION: i8 = 0;
    const DESC_REPRESENTATION: i8 = 1;
    const ASC: &'static str = "ASC";
    const DESC: &'static str = "DESC";

    pub fn new(sort_order: i8) -> Result<Self, ErrorAuditor> {
        todo!()

        // уалить ОрдерКОнвентионРезолвер
    }
}