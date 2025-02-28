use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                channel::{
                    Channel,
                    Channel_LinkedName,
                    Channel_Name,
                    Channel_ObfuscationValue,
                },
                user_access_token::UserAccessToken,
            },
            functionality::service::{
                extractor::{
                    Extracted,
                    Extractor,
                },
                validator::Validator,
                generator::Generator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    postgresql::{
                        ChannelBy2,
                        ChannelBy3,
                        ChannelInsert,
                        Postgresql,
                    },
                    Repository,
                },
                service::resolver::{
                    Resolver,
                    UnixTime,
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel::create::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct Channel_Create;
impl ActionProcessor_ for ActionProcessor<Channel_Create> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            let user__id = match Extractor::<UserAccessToken>::extract(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_signed,
            )? {
                Extracted::Data {
                    user_access_token__id: _,
                    user__id: user__id_,
                    user_device__id: _,
                    user_access_token__expires_at: _,
                } => user__id_,
                Extracted::AlreadyExpired => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired));
                }
                Extracted::InUserAccessTokenBlackList => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_InUserAccessTokenBlackList));
                }
            };
            if !Validator::<Channel_Name>::is_valid(incoming.channel__name) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<Channel_LinkedName>::is_valid(incoming.channel__linked_name) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_database_1_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await);
            if Repository::<Postgresql<Channel>>::is_exist_1(
                &postgresql_database_1_client,
                ChannelBy2 {
                    channel__name: incoming.channel__name,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel_NameAlreadyExist));
            }
            if Repository::<Postgresql<Channel>>::is_exist_2(
                &postgresql_database_1_client,
                ChannelBy3 {
                    channel__linked_name: incoming.channel__linked_name,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel_LinkedNameAlreadyExist));
            }
            let channel__id = Repository::<Postgresql<Channel>>::create(
                &postgresql_database_1_client,
                ChannelInsert {
                    channel__owner: user__id,
                    channel__name: incoming.channel__name,
                    channel__linked_name: incoming.channel__linked_name,
                    channel__description: Option::None,
                    channel__access_modifier: incoming.channel__access_modifier,
                    channel__visability_modifier: incoming.channel__visability_modifier,
                    channel__orientation: vec![].as_slice(),
                    channel__cover_image_path: Option::None,
                    channel__background_image_path: Option::None,
                    channel__subscribers_quantity: 0,
                    channel__marks_quantity: 0,
                    channel__viewing_quantity: 0,
                    channel__obfuscation_value: Generator::<Channel_ObfuscationValue>::generate(),
                    channel__created_at: Resolver::<UnixTime>::get_now_in_seconds(),
                },
            )
            .await?;
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        channel__id,
                    },
                ),
            );
        };
    }
}