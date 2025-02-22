use {
    crate::{
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
                    postgresql::{
                        Postgresql,
                        UserRegistrationTokenBy1,
                        UserRegistrationTokenUpdate2,
                    },
                    Repository,
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
    std::future::Future,
};
pub struct UserAuthorization_SendEmailForRegister;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_SendEmailForRegister> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<User_Email>::is_valid(incoming.user__email.as_str())? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (
                user_registration_token__value,
                user_registration_token__is_approved,
                user_registration_token__expires_at,
                mut user_registration_token__can_be_resent_from,
            ) = match Repository::<Postgresql<UserRegistrationToken<'_>>>::find_3(
                &postgresql_database_2_client,
                UserRegistrationTokenBy1 {
                    user__email: incoming.user__email.as_str(),
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(values) => values,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_NotFound));
                }
            };
            let now = Resolver::<UnixTime>::get_now_in_seconds();
            if user_registration_token__expires_at <= now {
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
            if user_registration_token__is_approved {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_AlreadyApproved));
            }
            if user_registration_token__can_be_resent_from > now {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_TimeToResendHasNotCome));
            }
            user_registration_token__can_be_resent_from = Generator::<UserRegistrationToken_CanBeResentFrom>::generate(now)?;
            Repository::<Postgresql<UserRegistrationToken<'_>>>::update_2(
                &postgresql_database_2_client,
                UserRegistrationTokenUpdate2 {
                    user_registration_token__can_be_resent_from,
                },
                UserRegistrationTokenBy1 {
                    user__email: incoming.user__email.as_str(),
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?;
            let environment_configuration = inner.environment_configuration;
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    EmailSender::<UserRegistrationToken<'_>>::repeatable_send(
                        &environment_configuration.subject.resource.email_server,
                        user_registration_token__value.as_str(),
                        incoming.user__email.as_str(),
                        incoming.user_device__id.as_str(),
                    )
                    .await?;
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
