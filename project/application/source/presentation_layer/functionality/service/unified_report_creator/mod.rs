use crate::presentation_layer::data::data_transfer_object::_in_context_for::presentation_layer::functionality::service::controller::_new_for_context::unified_report::unified_report::UnifiedReport;
use serde::Serialize;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Deserialize;

pub struct UnifiedReportCreator;

impl UnifiedReportCreator {
    pub fn create_without_data(
    ) -> UnifiedReport<()> {
        return UnifiedReport::new_without_data();
    }
}

#[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
impl UnifiedReportCreator {
    pub fn create_with_data<S>(
        data: S
    ) -> UnifiedReport<S>
    where
        S: Serialize
    {
        return UnifiedReport::new_with_data(data);
    }

    pub fn create_with_communication_code(
        communication_code: &'static str
    ) -> UnifiedReport<()> {
        return UnifiedReport::new_with_communication_code(communication_code)
    }
}

#[cfg(feature="facilitate_non_automatic_functional_testing")]
impl UnifiedReportCreator {
    pub fn create_with_data<S>(
        data: S
    ) -> UnifiedReport<S>
    where
        S: Serialize + for<'de> Deserialize<'de>
    {
        return UnifiedReport::new_with_data(data);
    }

    pub fn create_with_communication_code(
        communication_code: &'static str
    ) -> UnifiedReport<()> {
        return UnifiedReport::new_with_communication_code(communication_code.to_string());
    }
}