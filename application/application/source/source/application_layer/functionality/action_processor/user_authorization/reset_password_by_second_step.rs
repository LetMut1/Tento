use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            user::User_Id,
            user_device::UserDevice_Id,
            user_reset_password_token::{
                UserResetPasswordToken,
                UserResetPasswordToken_Value,
                UserResetPasswordToken_WrongEnterTriesQuantity,
            },
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                OptionConverter,
            },
            capture::Capture,
        },
        functionality::{
            repository::postgresql::{
                user_reset_password_token::{
                    By1,
                    Update4,
                    Update5,
                },
                PostgresqlRepository,
            },
            service::resolver::{
                Expiration,
                Resolver,
            },
        },
    },
};
use dedicated_crate::{
    action_processor_incoming_outcoming::action_processor::user_authorization::reset_password_by_second_step::{
        Incoming,
        Precedent,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::future::Future;
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
pub struct UserAuthorization_ResetPasswordBySecondStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_ResetPasswordBySecondStep> {
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
            if !Validator::<UserResetPasswordToken_Value>::is_valid(incoming.user_reset_password_token__value.as_str())? {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<User_Id>::is_valid(incoming.user__id) {
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
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
            let mut user_reset_password_token = match PostgresqlRepository::<UserResetPasswordToken>::find_2(
                database_2_postgresql_connection,
                By1 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(user_reset_password_token_) => user_reset_password_token_,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_NotFound));
                }
            };
            if Resolver::<Expiration>::is_expired(user_reset_password_token.expires_at) {
                PostgresqlRepository::<UserResetPasswordToken<'_>>::delete_2(
                    database_2_postgresql_connection,
                    By1 {
                        user__id: incoming.user__id,
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_AlreadyExpired));
            }
            if user_reset_password_token.is_approved {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_AlreadyApproved));
            }
            if user_reset_password_token.value != incoming.user_reset_password_token__value {
                user_reset_password_token.wrong_enter_tries_quantity = user_reset_password_token.wrong_enter_tries_quantity.checked_add(1).into_logic_out_of_range(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
                if user_reset_password_token.wrong_enter_tries_quantity < UserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                    PostgresqlRepository::<UserResetPasswordToken>::update_4(
                        database_2_postgresql_connection,
                        Update4 {
                            user_reset_password_token__wrong_enter_tries_quantity: user_reset_password_token.wrong_enter_tries_quantity,
                        },
                        By1 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    PostgresqlRepository::<UserResetPasswordToken<'_>>::delete_2(
                        database_2_postgresql_connection,
                        By1 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                }
                return Result::Ok(
                    UnifiedReport::precedent(
                        Precedent::UserResetPasswordToken_WrongValue {
                            user_reset_password_token__wrong_enter_tries_quantity: user_reset_password_token.wrong_enter_tries_quantity,
                        },
                    ),
                );
            }
            user_reset_password_token.is_approved = true;
            PostgresqlRepository::<UserResetPasswordToken>::update_5(
                database_2_postgresql_connection,
                Update5 {
                    user_reset_password_token__is_approved: user_reset_password_token.is_approved,
                },
                By1 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?;
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
