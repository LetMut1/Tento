use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::expiration_time_resolver::ExpirationTimeResolver;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::data::data_transfer_object::_in_context_for::infrastructure_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::extractor::_new_for_context::result::Result as ExtractorResult;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user_access_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserAccessTokenBlackListDataProviderRedis;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use redis::aio::Connection;

pub struct Extractor;

impl Extractor {
    pub async fn extract<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        application_user_access_token_classic_form: &'a str,
        connection: &'a mut Connection
    ) -> Result<ExtractorResult, ErrorAuditor> {
        match SerializationFormResolver::deserialize(environment_configuration_resolver, application_user_access_token_classic_form) {
            Ok(application_user_access_token) => {
                match ExpirationTimeResolver::is_expired(&application_user_access_token) {
                    Ok(is_expired) => {
                        if !is_expired {
// TODO TODO НУЖНО ЛИ здесь это? Может, не использовать блэелист вообще? При разлогине клиент сам удаляет токен. При массовом разлогине всем клиентам идет пуш, который так же удаляет токен
                            match ApplicationUserAccessTokenBlackListDataProviderRedis::is_exist_1(connection, application_user_access_token.get_id()).await {
                                Ok(is_exist_by_application_user_access_token_id) => {
                                    if !is_exist_by_application_user_access_token_id {
                                        return Ok(ExtractorResult::ApplicationUserAccessToken { application_user_access_token });
                                    }

                                    return Ok(ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList);
                                }
                                Err(mut error) => {
                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                    return Err(error);
                                }
                            }
                        }

                        return Ok(ExtractorResult::ApplicationUserAccessTokenAlreadyExpired);
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