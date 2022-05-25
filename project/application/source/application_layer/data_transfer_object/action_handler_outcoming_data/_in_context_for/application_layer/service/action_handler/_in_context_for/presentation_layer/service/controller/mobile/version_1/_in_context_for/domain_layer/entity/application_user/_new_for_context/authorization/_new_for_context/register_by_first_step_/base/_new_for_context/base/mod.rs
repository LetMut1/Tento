use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::controller::_new_for_context::unified_report::unified_report::UnifiedReport;
use http::response::Parts;

pub struct Base {
    parts: Parts,
    convertible_data: Option<UnifiedReport<()>>
}

impl Base {
    pub fn new(
        parts: Parts,
        convertible_data: Option<UnifiedReport<()>>
    ) -> Self {
        return Self {
            parts,
            convertible_data
        };
    }

    pub fn into_inner(
        self
    ) -> (Parts, Option<UnifiedReport<()>>) {
        return (
            self.parts,
            self.convertible_data,
        );
    }
}