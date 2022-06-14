use crate::presentation_layer::data::data_transfer_object::_in_context_for::presentation_layer::service::controller::_new_for_context::unified_report::unified_report::UnifiedReport;
use http::response::Parts;

pub struct Base<T> {
    parts: Parts,
    convertible_data: Option<UnifiedReport<T>>
}

impl<T> Base<T> {
    pub fn new(
        parts: Parts,
        convertible_data: Option<UnifiedReport<T>>
    ) -> Self {
        return Self {
            parts,
            convertible_data
        };
    }

    pub fn into_inner(
        self
    ) -> (Parts, Option<UnifiedReport<T>>) {
        return (
            self.parts,
            self.convertible_data,
        );
    }
}