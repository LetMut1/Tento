use super::by::By9;
use super::insert::Insert9;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::channel_outer_link::ChannelOuterLink;
use crate::domain_layer::data::entity::channel_outer_link::ChannelOuterLink_Address;
use crate::domain_layer::data::entity::channel_outer_link::ChannelOuterLink_Alias;
use crate::domain_layer::data::entity::channel_outer_link::ChannelOuterLink_CreatedAt;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;
use crate::infrastructure_layer::data::auditor::Converter;
pub use action_processor_incoming_outcoming::ChannelOuterLink1;

impl PostgresqlRepository<ChannelOuterLink> {
    pub async fn create<'a>(
        database_1_connection: &'a Connection,
        insert_9: Insert9,
    ) -> Result<ChannelOuterLink, Auditor<Error>> {
        let channel_outer_link_alias = insert_9.channel_outer_link_alias.0.as_str();

        let channel_outer_link_address = insert_9.channel_outer_link_address.0.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            INSERT INTO public.channel_inner_link AS cil ( \
                from_, \
                alias, \
                address, \
                created_at \
            ) VALUES ( \
                $1, \
                $2, \
                $3, \
                current_timestamp(6) \
            ) \
            RETURNING \
                cs.created_at::TEXT AS ca;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &insert_9.channel_outer_link_from.0,
                Type::INT8,
            )
            .add_parameter(
                &channel_outer_link_alias,
                Type::TEXT,
            )
            .add_parameter(
                &channel_outer_link_address,
                Type::TEXT,
            );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(BacktracePart::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(BacktracePart::new(line!(), file!()))?;

        let channel_outer_link = ChannelOuterLink {
            from: insert_9.channel_outer_link_from,
            alias: insert_9.channel_outer_link_alias,
            address: insert_9.channel_outer_link_address,
            created_at: ChannelOuterLink_CreatedAt(row_registry[0].try_get::<'_, usize, String>(0).convert(BacktracePart::new(line!(), file!()))?),
        };

        return Ok(channel_outer_link);
    }

    pub async fn find_1<'a>(
        database_1_connection: &'a Connection,
        by_9: &'a By9,
        limit: i16,
    ) -> Result<Vec<ChannelOuterLink1>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                col.alias AS al, \
                col.address AS ad \
            FROM public.channel_outer_link col \
            WHERE col.from_ = $1 \
            LIMIT $2";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_9.channel_outer_link_from.0,
                Type::INT8,
            )
            .add_parameter(
                &limit,
                Type::INT2,
            );

        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(BacktracePart::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(BacktracePart::new(line!(), file!()))?;

        let mut channel_outer_link_registry: Vec<ChannelOuterLink1> = vec![];

        if row_registry.is_empty() {
            return Ok(channel_outer_link_registry);
        }

        '_a: for row in row_registry.iter() {
            let channel_outer_link = ChannelOuterLink1 {
                channel_outer_link_alias: ChannelOuterLink_Alias(row.try_get::<'_, usize, String>(0).convert(BacktracePart::new(line!(), file!()))?),
                channel_outer_link_address: ChannelOuterLink_Address(row.try_get::<'_, usize, String>(1).convert(BacktracePart::new(line!(), file!()))?),
            };

            channel_outer_link_registry.push(channel_outer_link);
        }

        return Ok(channel_outer_link_registry);
    }
}
