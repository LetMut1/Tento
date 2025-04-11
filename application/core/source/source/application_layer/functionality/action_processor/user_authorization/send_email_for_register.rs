use {
    crate::{
        BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY,
        BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_QUANTITY,
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
                    UserRegistrationToken_CanBeResentFrom,
                },
            },
            functionality::service::{
                email_sender::EmailSender,
                generator::Generator,
                validator::Validator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        Postgresql,
                        UserRegistrationTokenBy,
                        UserRegistrationTokenUpdate2,
                    },
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
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user_authorization::send_email_for_register::{
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
pub struct UserAuthorization_SendEmailForRegister;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_SendEmailForRegister> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<User_Email>::is_valid(incoming.user__email)? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_client_database_2 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (user_registration_token__value, user_registration_token__is_approved, user_registration_token__expires_at, mut user_registration_token__can_be_resent_from) =
                match Repository::<Postgresql<UserRegistrationToken>>::find_3(
                    &postgresql_client_database_2,
                    UserRegistrationTokenBy {
                        user__email: incoming.user__email,
                        user_device__id: incoming.user_device__id,
                    },
                )
                .await?
                {
                    Option::Some(values) => values,
                    Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken__NotFound)),
                };
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
            if user_registration_token__expires_at <= now {
                if !Repository::<Postgresql<UserRegistrationToken>>::delete(
                    &postgresql_client_database_2,
                    UserRegistrationTokenBy {
                        user__email: incoming.user__email,
                        user_device__id: incoming.user_device__id,
                    },
                )
                .await?
                {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                }
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken__AlreadyExpired));
            }
            if user_registration_token__is_approved {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken__AlreadyApproved));
            }
            if user_registration_token__can_be_resent_from > now {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken__TimeToResendHasNotCome));
            }
            user_registration_token__can_be_resent_from = Generator::<UserRegistrationToken_CanBeResentFrom>::generate(now)?;
            if !Repository::<Postgresql<UserRegistrationToken>>::update_2(
                &postgresql_client_database_2,
                UserRegistrationTokenUpdate2 {
                    user_registration_token__can_be_resent_from,
                },
                UserRegistrationTokenBy {
                    user__email: incoming.user__email,
                    user_device__id: incoming.user_device__id,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
            }
            let email_server = &inner.environment_configuration.subject.resource.email_server;
            let user__email = incoming.user__email.to_string();
            let user_device__id = incoming.user_device__id.to_string();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    let mut interval = tokio::time::interval(Duration::from_secs(BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY));
                    '_a: for quantity in 1..=BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_QUANTITY {
                        interval.tick().await;
                        match EmailSender::<UserRegistrationToken>::send(
                            email_server,
                            user_registration_token__value.as_str(),
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
                user_registration_token__can_be_resent_from,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
