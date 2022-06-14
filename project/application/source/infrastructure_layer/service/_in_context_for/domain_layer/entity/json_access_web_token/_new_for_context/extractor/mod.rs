use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::expiration_time_resolver::ExpirationTimeResolver;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::extractor::_new_for_context::result::Result as ExtractorResult;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonAccessWebTokenBlackListDataProviderRedis;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use redis::aio::Connection;

pub struct Extractor;

impl Extractor {
    pub async fn extract<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        json_access_web_token_classic_form: &'a str,
        connection: &'a mut Connection
    ) -> Result<ExtractorResult, ErrorAuditor> {
        match SerializationFormResolver::deserialize(environment_configuration_resolver, json_access_web_token_classic_form) {
            Ok(json_access_web_token) => {
                match ExpirationTimeResolver::is_expired(&json_access_web_token) {
                    Ok(is_expired) => {
                        if !is_expired {
                            match JsonAccessWebTokenBlackListDataProviderRedis::is_exist_by_json_access_token_id(connection, json_access_web_token.get_id()).await {
                                Ok(is_exist_by_json_access_token_id) => {
                                    if !is_exist_by_json_access_token_id {
                                        return Ok(ExtractorResult::JsonAccessWebToken { json_access_web_token });
                                    }
                        
                                    return Ok(ExtractorResult::JsonAccessWebTokenInJsonAccessWebTokenBlackList);
                                }
                                Err(mut error) => {
                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                    
                                    return Err(error);
                                }
                            }
                        }
                
                        return Ok(ExtractorResult::JsonAccessWebTokenAlreadyExpired);
                    }
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
        
                        return Err(error);
                    }
                }
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}