use super::PostgresqlRepository;
use crate::{
    domain_layer::data::entity::channel_inner_link::ChannelInnerLink,
    infrastructure_layer::{
        data::{
            auditor::{
                Auditor,
                Backtrace,
                ResultConverter,
            },
            error::Error,
        },
        functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver,
    },
};
pub use action_processor_incoming_outcoming::ChannelInnerLink1;
use tokio_postgres::{
    types::Type,
    Client as Connection,
};
impl PostgresqlRepository<ChannelInnerLink> {
    pub async fn create_1<'a>(database_1_connection: &'a Connection, insert_1: Insert1) -> Result<ChannelInnerLink, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
        let query = "\
            INSERT INTO public.channel_inner_link AS cil ( \
                from_, \
                to_, \
                created_at \
            ) VALUES ( \
                $1, \
                $2, \
                current_timestamp(6) \
            ) \
            RETURNING \
                cs.created_at::TEXT AS ca;";
        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&insert_1.channel_inner_link__from, Type::INT8)
            .add_parameter(&insert_1.channel_inner_link__to, Type::INT8);
        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert_into_error(Backtrace::new(line!(), file!()))?;
        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert_into_error(Backtrace::new(line!(), file!()))?;
        return Ok(ChannelInnerLink::new(
            insert_1.channel_inner_link__from,
            insert_1.channel_inner_link__to,
            row_registry[0].try_get::<'_, usize, String>(0).convert_into_error(Backtrace::new(line!(), file!()))?,
        ));
    }
    pub async fn find_1<'a>(database_1_connection: &'a Connection, by_1: By1, limit: i16) -> Result<Vec<ChannelInnerLink1>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();
        let query = "\
            SELECT \
                cil.to_ AS t \
            FROM public.channel_inner_link cil \
            WHERE cil.from_ = $1 \
            LIMIT $2";
        prepared_statemant_parameter_convertation_resolver.add_parameter(&by_1.channel_inner_link__from, Type::INT8).add_parameter(&limit, Type::INT2);
        let statement = database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
            .convert_into_error(Backtrace::new(line!(), file!()))?;
        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
            .convert_into_error(Backtrace::new(line!(), file!()))?;
        let mut channel_inner_link_registry: Vec<ChannelInnerLink1> = vec![];
        if row_registry.is_empty() {
            return Ok(channel_inner_link_registry);
        }
        '_a: for row in row_registry.iter() {
            let channel_inner_link = ChannelInnerLink1 {
                channel_inner_link__to: row.try_get::<'_, usize, i64>(0).convert_into_error(Backtrace::new(line!(), file!()))?,
            };
            channel_inner_link_registry.push(channel_inner_link);
        }
        return Ok(channel_inner_link_registry);
    }
}
pub struct Insert1 {
    pub channel_inner_link__from: i64,
    pub channel_inner_link__to: i64,
}
pub struct By1 {
    pub channel_inner_link__from: i64,
}
