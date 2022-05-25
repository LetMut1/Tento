use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::controller::_new_for_context::unified_report::unified_report::UnifiedReport;
use crate::application_layer::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_last_step::base::_new_for_context::base::Base as ActionHandlerOutcomingData;
use http::response::Parts;

pub struct Base {
    parts: Parts,
    convertible_data: Option<UnifiedReport<ActionHandlerOutcomingData>>
}

impl Base {
    pub fn new(
        parts: Parts,
        convertible_data: Option<UnifiedReport<ActionHandlerOutcomingData>>
    ) -> Self {
        return Self {
            parts,
            convertible_data
        };
    }

    pub fn into_inner(
        self
    ) -> (Parts, Option<UnifiedReport<ActionHandlerOutcomingData>>) {
        return (
            self.parts,
            self.convertible_data,
        );
    }
}