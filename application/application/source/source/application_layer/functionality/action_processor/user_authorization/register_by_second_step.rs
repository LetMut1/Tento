use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            user::User_Email,
            user_device::UserDevice_Id,
            user_registration_token::{
                UserRegistrationToken,
                UserRegistrationToken_Value,
                UserRegistrationToken_WrongEnterTriesQuantity,
            },
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            capture::Capture,
        },
        functionality::{
            repository::{
                postgresql::{
                    Postgresql,
                    UserRegistrationTokenBy1,
                    UserRegistrationTokenUpdate5,
                },
                Repository,
            },
            service::resolver::{
                Resolver,
                UnixTime,
            },
        },
    },
};
use dedicated_crate::{
    action_processor_incoming_outcoming::action_processor::user_authorization::register_by_second_step::{
        Incoming,
        Precedent,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::future::Future;
pub struct UserAuthorization_RegisterBySecondStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_RegisterBySecondStep> {
    type Incoming = Incoming;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            if !Validator::<User_Email>::is_valid(incoming.user__email.as_str())? {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id.as_str()) {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<UserRegistrationToken_Value>::is_valid(incoming.user_registration_token__value.as_str())? {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            let postgresql_database_2_client = inner.postgresql_connection_pool_database_2.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
            let mut user_registration_token = match Repository::<Postgresql<UserRegistrationToken<'_>>>::find_2(
                &postgresql_database_2_client,
                UserRegistrationTokenBy1 {
                    user__email: incoming.user__email.as_str(),
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(user_registration_token_) => user_registration_token_,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_NotFound));
                }
            };
            if user_registration_token.expires_at <= Resolver::<UnixTime>::get_now() {
                Repository::<Postgresql<UserRegistrationToken<'_>>>::delete_2(
                    &postgresql_database_2_client,
                    UserRegistrationTokenBy1 {
                        user__email: incoming.user__email.as_str(),
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_AlreadyExpired));
            }
            if user_registration_token.is_approved {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_AlreadyApproved));
            }
            if user_registration_token.value != incoming.user_registration_token__value {
                if user_registration_token.wrong_enter_tries_quantity < UserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                    user_registration_token.wrong_enter_tries_quantity += 1;
                }
                if user_registration_token.wrong_enter_tries_quantity < UserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                    Repository::<Postgresql<UserRegistrationToken<'_>>>::update_4(
                        &postgresql_database_2_client,
                        UserRegistrationTokenBy1 {
                            user__email: incoming.user__email.as_str(),
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    Repository::<Postgresql<UserRegistrationToken<'_>>>::delete_2(
                        &postgresql_database_2_client,
                        UserRegistrationTokenBy1 {
                            user__email: incoming.user__email.as_str(),
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                }
                return Result::Ok(
                    UnifiedReport::precedent(
                        Precedent::UserRegistrationToken_WrongValue {
                            user_registration_token__wrong_enter_tries_quantity: user_registration_token.wrong_enter_tries_quantity,
                        },
                    ),
                );
            }
            user_registration_token.is_approved = true;
            Repository::<Postgresql<UserRegistrationToken<'_>>>::update_5(
                &postgresql_database_2_client,
                UserRegistrationTokenUpdate5 {
                    user_registration_token__is_approved: user_registration_token.is_approved,
                },
                UserRegistrationTokenBy1 {
                    user__email: incoming.user__email.as_str(),
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?;
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
