use super::by::By11;
use super::by::By12;
use super::by::By13;
use super::PostgresqlRepository;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use super::channel::Channel1;
use crate::infrastructure_layer::functionality::service::counter::Counter;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
pub use action_processor_incoming_outcoming::Common1;
use tokio_postgres::types::Type;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use tokio_postgres::Client as Connection;

impl PostgresqlRepository<Common1> {
    pub async fn find_1<'a>(
        database_1_connection: &'a Connection,
        by_11: &'a By11<'_>,
        limit: i16,
    ) -> Result<Vec<Common1>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::<i16>::new_classic();

        let mut query = format!(
            "SELECT \
                c.id AS i, \
                c.name AS n, \
                c.linked_name AS ln, \
                c.access_modifier AS am, \
                c.cover_image_path AS cip, \
                c.background_image_path AS bip, \
                cs.channel_id AS ca \
            FROM public.channel c LEFT OUTER JOIN public.channel_subscription cs \
            ON cs.application_user_id = ${} AND c.id = cs.channel_id \
            WHERE c.visability_modifier = ${} AND c.name LIKE ${}",
            counter.get_next_value()?,
            counter.get_next_value()?,
            counter.get_next_value()?,
        );

        let wildcard = format!(
            "{}%",
            by_11.channel_name,
        );

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_11.application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &by_11.channel_visability_modifier,
                Type::INT2,
            )
            .add_parameter(
                &wildcard,
                Type::TEXT,
            );

        let requery_channel_name: &'_ str;

        if let Some(requery_channel_name_) = by_11.requery_channel_name {
            requery_channel_name = requery_channel_name_.as_str();

            query = format!(
                "{} AND c.name > ${}",
                query.as_str(),
                counter.get_next_value()?,
            );

            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &requery_channel_name,
                Type::TEXT,
            );
        }

        query = format!(
            "{} \
            ORDER BY c.name ASC \
            LIMIT ${};",
            query.as_str(),
            counter.get_next_value()?
        );

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &limit,
            Type::INT2,
        );

        let statement = database_1_connection
            .prepare_typed(
                query.as_str(),
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(Backtrace::new(line!(), file!()))?;

        let mut common_registry: Vec<Common1> = vec![];

        if row_registry.is_empty() {
            return Ok(common_registry);
        }

        '_a: for row in row_registry.iter() {
            let channel = Channel1 {
                channel_id: row.try_get::<'_, usize, i64>(0).convert(Backtrace::new(line!(), file!()))?,
                channel_name: row.try_get::<'_, usize, String>(1).convert(Backtrace::new(line!(), file!()))?,
                channel_linked_name: row.try_get::<'_, usize, String>(2).convert(Backtrace::new(line!(), file!()))?,
                channel_access_modifier: row.try_get::<'_, usize, i16>(3).convert(Backtrace::new(line!(), file!()))?,
                channel_visability_modifier: by_11.channel_visability_modifier,
                channel_cover_image_path: row.try_get::<'_, usize, Option<String>>(4).convert(Backtrace::new(line!(), file!()))?,
                channel_background_image_path: row.try_get::<'_, usize, Option<String>>(5).convert(Backtrace::new(line!(), file!()))?,
            };

            let is_application_user_subscribed = match row.try_get::<'_, usize, Option<i64>>(6).convert(Backtrace::new(line!(), file!()))? {
                Some(_) => true,
                None => false,
            };

            let common = Common1 {
                channel,
                is_application_user_subscribed,
            };

            common_registry.push(common);
        }

        return Ok(common_registry);
    }

    pub async fn find_2<'a>(
        database_1_connection: &'a Connection,
        by_12: &'a By12<'_>,
        limit: i16,
    ) -> Result<Vec<Common1>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::<i16>::new_classic();

        let mut query = format!(
            "SELECT \
                c.id AS i, \
                c.name AS n, \
                c.linked_name AS ln, \
                c.access_modifier AS am, \
                c.visability_modifier AS vm, \
                c.cover_image_path AS cip, \
                c.background_image_path AS bip \
            FROM public.channel c INNER JOIN public.channel_subscription cs \
            ON cs.application_user_id = ${} AND c.id = cs.channel_id \
            WHERE c.name LIKE ${}",
            counter.get_next_value()?,
            counter.get_next_value()?,
        );

        let wildcard = format!(
            "{}%",
            by_12.channel_name,
        );

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_12.application_user_id,
                Type::INT8,
            )
            .add_parameter(
                &wildcard,
                Type::TEXT,
            );

        let requery_channel_name: &'_ str;

        if let Some(requery_channel_name_) = by_12.requery_channel_name {
            requery_channel_name = requery_channel_name_.as_str();

            query = format!(
                "{} AND c.name > ${}",
                query.as_str(),
                counter.get_next_value()?,
            );

            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &requery_channel_name,
                Type::TEXT,
            );
        }

        query = format!(
            "{} \
            ORDER BY c.name ASC \
            LIMIT ${};",
            query.as_str(),
            counter.get_next_value()?,
        );

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &limit,
            Type::INT2,
        );

        let statement = database_1_connection
            .prepare_typed(
                query.as_str(),
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(Backtrace::new(line!(), file!()))?;

        let mut common_registry: Vec<Common1> = vec![];

        if row_registry.is_empty() {
            return Ok(common_registry);
        }

        '_a: for row in row_registry.iter() {
            let channel = Channel1 {
                channel_id: row.try_get::<'_, usize, i64>(0).convert(Backtrace::new(line!(), file!()))?,
                channel_name: row.try_get::<'_, usize, String>(1).convert(Backtrace::new(line!(), file!()))?,
                channel_linked_name: row.try_get::<'_, usize, String>(2).convert(Backtrace::new(line!(), file!()))?,
                channel_access_modifier: row.try_get::<'_, usize, i16>(3).convert(Backtrace::new(line!(), file!()))?,
                channel_visability_modifier: row.try_get::<'_, usize, i16>(4).convert(Backtrace::new(line!(), file!()))?,
                channel_cover_image_path: row.try_get::<'_, usize, Option<String>>(5).convert(Backtrace::new(line!(), file!()))?,
                channel_background_image_path: row.try_get::<'_, usize, Option<String>>(6).convert(Backtrace::new(line!(), file!()))?,
            };

            let common = Common1 {
                channel,
                is_application_user_subscribed: true,
            };

            common_registry.push(common);
        }

        return Ok(common_registry);
    }

    pub async fn find_3<'a>(
        database_1_connection: &'a Connection,
        by_13: &'a By13,
        limit: i16,
    ) -> Result<Vec<Common1>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let mut counter = Counter::<i16>::new_classic();

        let mut query = format!(
            "SELECT \
                c.id AS i, \
                c.name AS n, \
                c.linked_name AS ln, \
                c.access_modifier AS am, \
                c.visability_modifier AS vm, \
                c.cover_image_path AS cip, \
                c.background_image_path AS bip \
            FROM public.channel c INNER JOIN public.channel_subscription cs \
            ON cs.application_user_id = ${} AND c.id = cs.channel_id",
            counter.get_next_value()?,
        );

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &by_13.application_user_id,
            Type::INT8,
        );

        let requery_channel_id: i64;

        if let Some(requery_channel_id_) = by_13.requery_channel_id {
            requery_channel_id = requery_channel_id_;

            query = format!(
                "{} \
                WHERE cs.channel_id > ${}",
                query.as_str(),
                counter.get_next_value()?,
            );

            prepared_statemant_parameter_convertation_resolver.add_parameter(
                &requery_channel_id,
                Type::INT8,
            );
        }

        query = format!(
            "{} \
            ORDER BY cs.channel_id ASC \
            LIMIT ${};",
            query.as_str(),
            counter.get_next_value()?,
        );

        prepared_statemant_parameter_convertation_resolver.add_parameter(
            &limit,
            Type::INT2,
        );

        let statement = database_1_connection
            .prepare_typed(
                query.as_str(),
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        .convert(Backtrace::new(line!(), file!()))?;

        let row_registry = database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
.convert(Backtrace::new(line!(), file!()))?;

        let mut common_registry: Vec<Common1> = vec![];

        if row_registry.is_empty() {
            return Ok(common_registry);
        }

        '_a: for row in row_registry.iter() {
            let channel = Channel1 {
                channel_id: row.try_get::<'_, usize, i64>(0).convert(Backtrace::new(line!(), file!()))?,
                channel_name: row.try_get::<'_, usize, String>(1).convert(Backtrace::new(line!(), file!()))?,
                channel_linked_name: row.try_get::<'_, usize, String>(2).convert(Backtrace::new(line!(), file!()))?,
                channel_access_modifier: row.try_get::<'_, usize, i16>(3).convert(Backtrace::new(line!(), file!()))?,
                channel_visability_modifier: row.try_get::<'_, usize, i16>(4).convert(Backtrace::new(line!(), file!()))?,
                channel_cover_image_path: row.try_get::<'_, usize, Option<String>>(5).convert(Backtrace::new(line!(), file!()))?,
                channel_background_image_path: row.try_get::<'_, usize, Option<String>>(6).convert(Backtrace::new(line!(), file!()))?,
            };

            let common = Common1 {
                channel,
                is_application_user_subscribed: true,
            };

            common_registry.push(common);
        }

        return Ok(common_registry);
    }
}
