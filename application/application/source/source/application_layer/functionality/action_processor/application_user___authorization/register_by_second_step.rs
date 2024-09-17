use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            application_user::ApplicationUser_Email,
            application_user_device::ApplicationUserDevice_Id,
            application_user_registration_token::{
                ApplicationUserRegistrationToken,
                ApplicationUserRegistrationToken_Value,
                ApplicationUserRegistrationToken_WrongEnterTriesQuantity,
            },
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::capture::Capture,
        functionality::{
            repository::postgresql::{
                application_user_registration_token::{
                    By1,
                    Update4,
                    Update5,
                },
                PostgresqlRepository,
            },
            service::resolver::{
                expiration::Expiration,
                Resolver,
            },
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_second_step::{
    Incoming,
    Precedent,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    OptionConverter,
};
use std::future::Future;
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use unified_report::UnifiedReport;
use void::Void;
pub struct ApplicationUser__Authorization___RegisterBySecondStep;
impl ActionProcessor_ for ActionProcessor<ApplicationUser__Authorization___RegisterBySecondStep> {
    type Incoming = Incoming;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a, T>(
        inner: &'a Inner<'_, T>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return async move {
            if !Validator::<ApplicationUser_Email>::is_valid(incoming.application_user__email.as_str())? {
                return Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming.application_user_device__id.as_str()) {
                return Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<ApplicationUserRegistrationToken_Value>::is_valid(incoming.application_user_registration_token__value.as_str())? {
                return Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
            let mut application_user_registration_token = match PostgresqlRepository::<ApplicationUserRegistrationToken>::find_2(
                database_2_postgresql_connection,
                By1 {
                    application_user__email: incoming.application_user__email.as_str(),
                    application_user_device__id: incoming.application_user_device__id.as_str(),
                },
            )
            .await?
            {
                Some(application_user_registration_token_) => application_user_registration_token_,
                None => {
                    return Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_NotFound));
                }
            };
            if Resolver::<Expiration>::is_expired(application_user_registration_token.expires_at) {
                PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete_2(
                    database_2_postgresql_connection,
                    By1 {
                        application_user__email: incoming.application_user__email.as_str(),
                        application_user_device__id: incoming.application_user_device__id.as_str(),
                    },
                )
                .await?;
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_AlreadyExpired));
            }
            if application_user_registration_token.is_approved {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_AlreadyApproved));
            }
            if application_user_registration_token.value != incoming.application_user_registration_token__value {
                application_user_registration_token.wrong_enter_tries_quantity =
                    application_user_registration_token.wrong_enter_tries_quantity.checked_add(1).into_logic_out_of_range(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?;
                if application_user_registration_token.wrong_enter_tries_quantity < ApplicationUserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                    PostgresqlRepository::<ApplicationUserRegistrationToken>::update_4(
                        database_2_postgresql_connection,
                        Update4 {
                            application_user_registration_token__wrong_enter_tries_quantity: application_user_registration_token.wrong_enter_tries_quantity,
                        },
                        By1 {
                            application_user__email: incoming.application_user__email.as_str(),
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete_2(
                        database_2_postgresql_connection,
                        By1 {
                            application_user__email: incoming.application_user__email.as_str(),
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                }
                return Ok(
                    UnifiedReport::precedent(
                        Precedent::ApplicationUserRegistrationToken_WrongValue {
                            application_user_registration_token__wrong_enter_tries_quantity: application_user_registration_token.wrong_enter_tries_quantity,
                        },
                    ),
                );
            }
            application_user_registration_token.is_approved = true;
            PostgresqlRepository::<ApplicationUserRegistrationToken>::update_5(
                database_2_postgresql_connection,
                Update5 {
                    application_user_registration_token__is_approved: application_user_registration_token.is_approved,
                },
                By1 {
                    application_user__email: incoming.application_user__email.as_str(),
                    application_user_device__id: incoming.application_user_device__id.as_str(),
                },
            )
            .await?;
            return Ok(UnifiedReport::target_empty());
        };
    }
}
