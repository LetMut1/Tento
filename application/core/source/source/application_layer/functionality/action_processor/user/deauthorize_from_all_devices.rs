use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                user_access_refresh_token::UserAccessRefreshToken,
                user_access_token::UserAccessToken,
            },
            functionality::service::encoder::Encoder,
        },
        infrastructure_layer::{
            data::{aggregate_error::AggregateError, sended::Sended_},
            functionality::{
                repository::{
                    postgresql::{
                        Postgresql,
                        UserAccessRefreshTokenBy1,
                    }, Repository
                },
                service::{resolver::{
                    CloudMessage,
                    Resolver,
                    UnixTime,
                }, task_spawner::TaskSpawner},
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user::deauthorize_from_all_devices::{
            Incoming,
            Precedent,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::future::Future,
};
pub struct DeauthorizeFromAllDevices;
impl ActionProcessor_ for ActionProcessor<DeauthorizeFromAllDevices> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
            let private_key = &inner.environment_configuration.subject.encryption.private_key;
            let sended = Sended_::new(&raw const incoming as *const Self::Incoming<'static>);
            if !crate::result_return_runtime!(
                TaskSpawner::spawn_rayon_task_processed(
                    move || -> _ {
                        return Encoder::<UserAccessToken>::is_valid(
                            private_key,
                            &(unsafe { sended.read_() }).user_access_token_signed,
                        );
                    },
                ).await
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if incoming.user_access_token_signed.user_access_token__expires_at <= now {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken__AlreadyExpired));
            }
            let _ = Repository::<Postgresql<UserAccessRefreshToken>>::delete_2(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await),
                UserAccessRefreshTokenBy1 {
                    user__id: incoming.user_access_token_signed.user__id,
                },
            )
            .await?;
            Resolver::<CloudMessage>::deauthorize_user_from_all_devices();
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
