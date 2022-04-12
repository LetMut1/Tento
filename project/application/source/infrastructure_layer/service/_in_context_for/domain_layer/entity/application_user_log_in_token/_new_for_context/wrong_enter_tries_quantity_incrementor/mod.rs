use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::wrong_enter_tries_quantity_incrementor_trait::WrongEnterTriesQuantityIncrementorTrait;
use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;

pub struct WrongEnterTriesQuantityIncrementor;

impl WrongEnterTriesQuantityIncrementorTrait for WrongEnterTriesQuantityIncrementor {
    type Error = ErrorAggregator;
}