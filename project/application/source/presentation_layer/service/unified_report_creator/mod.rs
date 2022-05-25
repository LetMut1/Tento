use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::controller::_new_for_context::unified_report::unified_report::UnifiedReport;
use serde::Serialize;

pub struct UnifiedReportCreator;

impl UnifiedReportCreator {
    pub fn create_without_data(
    ) -> UnifiedReport<()> {
        return UnifiedReport::new_without_data();
    }

    pub fn create_with_data<S>(
        data: S
    ) -> UnifiedReport<S>
    where
        S: Serialize
    {
        return UnifiedReport::new_with_data(data);
    }

    #[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
    pub fn create_with_error_code(
        error_code: &'static str
    ) -> UnifiedReport<()> {
        return UnifiedReport::new_with_error_code(error_code)
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub fn create_with_error_code(
        error_code: &'static str
    ) -> UnifiedReport<()> {
        return UnifiedReport::new_with_error_code(error_code.to_string());
    }
}