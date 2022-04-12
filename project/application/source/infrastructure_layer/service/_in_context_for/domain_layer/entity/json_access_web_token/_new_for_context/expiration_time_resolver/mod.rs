use chrono::Utc;
use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::expiration_time_resolver_trait::ExpirationTimeResolverTrait;
use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::service::date_time_resolver::DateTimeResolver;

pub struct ExpirationTimeResolver;

impl ExpirationTimeResolverTrait for ExpirationTimeResolver {
    type Error = ErrorAggregator;

    fn is_expired<'a>(
        json_access_web_token: &'a JsonAccessWebToken<'_>
    ) -> Result<bool, Self::Error> {
        return Ok(
            !DateTimeResolver::is_greater_or_equal_than(
                &DateTimeResolver::create_chrono_date_time_utc(json_access_web_token.get_expiration_time())?,
                &Utc::now()
            )
        );
    }
}