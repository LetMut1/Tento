use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        }, domain_layer::{
            data::entity::{
                user::{
                    User,
                    User_ObfuscatedId,
                },
                user_authorization_token::{
                    UserAuthorizationToken,
                    UserAuthorizationToken_CanBeResentFrom,
                },
                user_device::UserDevice_Id,
            },
            functionality::service::{
                email_sender::EmailSender,
                generator::Generator,
                validator::Validator,
            },
        }, infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    postgresql::{
                        Postgresql,
                        UserAuthorizationTokenBy,
                        UserAuthorizationTokenUpdate3,
                        UserBy4,
                    }, Repository
                },
                service::{
                    resolver::{
                        Resolver,
                        UnixTime,
                    },
                    spawner::{
                        Spawner,
                        TokioNonBlockingTask,
                    },
                },
            },
        }, BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY, BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_QUANTITY
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user::send_email_for_authorize::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::{
        future::Future,
        time::Duration,
    },
};
pub struct SendEmailForAuthorize;
impl ActionProcessor_ for ActionProcessor<SendEmailForAuthorize> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<User_ObfuscatedId>::is_valid(incoming.user__obfuscated_id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let user__email = match Repository::<Postgresql<User>>::find_6(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                UserBy4 {
                    user__obfuscated_id: incoming.user__obfuscated_id,
                },
            )
            .await?
            {
                Option::Some(user_) => user_,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::User__NotFound)),
            };
            let postgresql_client_database_2 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (user_authorization_token__value, user_authorization_token__expires_at, mut user_authorization_token__can_be_resent_from) =
                match Repository::<Postgresql<UserAuthorizationToken>>::find_3(
                    &postgresql_client_database_2,
                    UserAuthorizationTokenBy {
                        user__obfuscated_id: incoming.user__obfuscated_id,
                        user_device__id: incoming.user_device__id,
                    },
                )
                .await?
                {
                    Option::Some(values) => values,
                    Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken__NotFound)),
                };
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
            if user_authorization_token__expires_at <= now {
                if !Repository::<Postgresql<UserAuthorizationToken>>::delete(
                    &postgresql_client_database_2,
                    UserAuthorizationTokenBy {
                        user__obfuscated_id: incoming.user__obfuscated_id,
                        user_device__id: incoming.user_device__id,
                    },
                )
                .await?
                {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                }
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken__AlreadyExpired));
            }
            if user_authorization_token__can_be_resent_from > now {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken__TimeToResendHasNotCome));
            }
            user_authorization_token__can_be_resent_from = Generator::<UserAuthorizationToken_CanBeResentFrom>::generate(now)?;
            if !Repository::<Postgresql<UserAuthorizationToken>>::update_3(
                &postgresql_client_database_2,
                UserAuthorizationTokenUpdate3 {
                    user_authorization_token__can_be_resent_from,
                },
                UserAuthorizationTokenBy {
                    user__obfuscated_id: incoming.user__obfuscated_id,
                    user_device__id: incoming.user_device__id,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
            }
            let email_server = &inner.environment_configuration.subject.resource.email_server;
            let user_device__id = incoming.user_device__id.to_string();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    let mut interval = tokio::time::interval(Duration::from_secs(BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY));
                    '_a: for quantity in 1..=BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_QUANTITY {
                        interval.tick().await;
                        match EmailSender::<UserAuthorizationToken>::send(
                            email_server,
                            user_authorization_token__value.as_str(),
                            user__email.as_str(),
                            user_device__id.as_str(),
                        )
                        .await
                        {
                            Ok(_) => return Result::Ok(()),
                            Err(aggregate_error) => {
                                if quantity == BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_QUANTITY {
                                    return Err(aggregate_error);
                                }
                            }
                        }
                    }
                    return Result::Ok(());
                },
            );
            let outcoming = Outcoming {
                user_authorization_token__can_be_resent_from,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
