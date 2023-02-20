use crate::infrastructure_layer::data::void::Void;
use crate::presentation_layer::data::unified_report::UnifiedReport;
use extern_crate::serde::Serialize;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use extern_crate::serde::Deserialize;

pub struct UnifiedReportCreator;

impl UnifiedReportCreator {
    pub fn create_empty() -> UnifiedReport<Void> {
        return UnifiedReport::empty();
    }

    pub fn create_with_communication_code(communication_code: i64) -> UnifiedReport<Void> {
        return UnifiedReport::communication_code(communication_code);
    }
}

#[cfg(not(feature = "facilitate_non_automatic_functional_testing"))]
impl UnifiedReportCreator {
    pub fn create_with_data<S>(data: S) -> UnifiedReport<S>
    where
        S: Serialize
    {
        return UnifiedReport::data(data);
    }
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
impl UnifiedReportCreator {
    pub fn create_with_data<S>(data: S) -> UnifiedReport<S>
    where
        S: Serialize + for<'de> Deserialize<'de>
    {
        return UnifiedReport::data(data);
    }
}