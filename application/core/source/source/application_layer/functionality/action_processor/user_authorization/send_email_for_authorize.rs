use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                user::{
                    User,
                    User_Id,
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
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    postgresql::{
                        Postgresql,
                        UserAuthorizationTokenBy1,
                        UserAuthorizationTokenUpdate3,
                        UserBy3,
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
        action_processor_incoming_outcoming::action_processor::user_authorization::send_email_for_authorize::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct UserAuthorization_SendEmailForAuthorize;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_SendEmailForAuthorize> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<User_Id>::is_valid(incoming.user__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let user = match Repository::<Postgresql<User<'_>>>::find_6(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await),
                UserBy3 {
                    user__id: incoming.user__id,
                },
            )
            .await?
            {
                Option::Some(user_) => user_,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::User_NotFound));
                }
            };
            let postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let mut user_authorization_token = match Repository::<Postgresql<UserAuthorizationToken<'_>>>::find_3(
                &postgresql_database_2_client,
                UserAuthorizationTokenBy1 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(user_authorization_token_) => user_authorization_token_,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken_NotFound));
                }
            };
            let now = Resolver::<UnixTime>::get_now_in_seconds();
            if user_authorization_token.expires_at <= now {
                Repository::<Postgresql<UserAuthorizationToken<'_>>>::delete_1(
                    &postgresql_database_2_client,
                    UserAuthorizationTokenBy1 {
                        user__id: incoming.user__id,
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken_AlreadyExpired));
            }
            if user_authorization_token.can_be_resent_from > now {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken_TimeToResendHasNotCome));
            }
            user_authorization_token.can_be_resent_from = Generator::<UserAuthorizationToken_CanBeResentFrom>::generate(now)?;
            Repository::<Postgresql<UserAuthorizationToken<'_>>>::update_3(
                &postgresql_database_2_client,
                UserAuthorizationTokenUpdate3 {
                    user_authorization_token__can_be_resent_from: user_authorization_token.can_be_resent_from,
                },
                UserAuthorizationTokenBy1 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?;
            let environment_configuration = inner.environment_configuration;
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    EmailSender::<UserAuthorizationToken<'_>>::repeatable_send(
                        &environment_configuration.subject.resource.email_server,
                        user_authorization_token.value.as_str(),
                        user.email.as_str(),
                        incoming.user_device__id.as_str(),
                    )
                    .await?;
                    return Result::Ok(());
                },
            );
            let outcoming = Outcoming {
                user_authorization_token__can_be_resent_from: user_authorization_token.can_be_resent_from,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
