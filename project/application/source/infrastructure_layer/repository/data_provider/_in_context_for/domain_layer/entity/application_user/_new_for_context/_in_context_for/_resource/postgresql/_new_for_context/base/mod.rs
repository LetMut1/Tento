use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use tokio_postgres::Client as Connection;
use tokio_postgres::types::Type;

pub struct Base;

impl Base {
    pub async fn is_exist_by_nickanme<'a>(
        connection: &'a mut Connection,
        nickname: &'a str
    ) -> Result<bool, BaseError> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.nickname = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&nickname, Type::TEXT);

        let statement = connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await?;

        let row_registry = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await?;
        if row_registry.is_empty() {
            return Ok(false);
        }

        return Ok(true);
    }

    pub async fn is_exist_by_email<'a>(
        connection: &'a mut Connection,
        email: &'a str
    ) -> Result<bool, BaseError> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&email, Type::TEXT);

        let statement = connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await?;

        let row_registry = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await?;
        if row_registry.is_empty() {
            return Ok(false);
        }

        return Ok(true);
    }

    pub async fn find_by_email<'a>(
        connection: &'a mut Connection,
        email: &'a str
    ) -> Result<Option<ApplicationUser>, BaseError> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "SELECT \
                au.id AS i, \
                au.email AS e, \
                au.nickname AS n, \
                au.password_hash AS ph, \
                au.created_at::TEXT AS ca \
            FROM public.application_user au \
            WHERE au.email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&email, Type::TEXT);

        let statement = connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await?;

        let row_registry = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await?;
        if !row_registry.is_empty() {
            return Ok(Some(
                ApplicationUser::new(
                    Some(row_registry[0].try_get::<'_, usize, i64>(0)?),
                    row_registry[0].try_get::<'_, usize, String>(1)?,
                    row_registry[0].try_get::<'_, usize, String>(2)?,
                    row_registry[0].try_get::<'_, usize, String>(3)?,
                    row_registry[0].try_get::<'_, usize, String>(4)?
                )
            ));
        }

        return Ok(None);
    }

    pub async fn find_by_nickname<'a>(
        connection: &'a mut Connection,
        nickname: &'a str
    ) -> Result<Option<ApplicationUser>, BaseError> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "SELECT \
                au.id AS i, \
                au.email AS e, \
                au.nickname AS n, \
                au.password_hash AS ph, \
                au.created_at::TEXT AS ca \
            FROM public.application_user au \
            WHERE au.nickname = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&nickname, Type::TEXT);

        let statement = connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await?;

        let row_registry = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await?;
        if !row_registry.is_empty() {
            return Ok(Some(
                ApplicationUser::new(
                    Some(row_registry[0].try_get::<'_, usize, i64>(0)?),
                    row_registry[0].try_get::<'_, usize, String>(1)?,
                    row_registry[0].try_get::<'_, usize, String>(2)?,
                    row_registry[0].try_get::<'_, usize, String>(3)?,
                    row_registry[0].try_get::<'_, usize, String>(4)?
                )
            ));
        }

        return Ok(None);
    }

    pub async fn find_by_id<'a>(
        connection: &'a mut Connection,
        id: &'a i64
    ) -> Result<Option<ApplicationUser>, BaseError> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "SELECT \
                au.id AS i, \
                au.email AS e, \
                au.nickname AS n, \
                au.password_hash AS ph, \
                au.created_at::TEXT AS ca \
            FROM public.application_user au \
            WHERE au.id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(id, Type::INT8);

        let statement = connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await?;

        let row_registry = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await?;
        if !row_registry.is_empty() {
            return Ok(Some(
                ApplicationUser::new(
                    Some(row_registry[0].try_get::<'_, usize, i64>(0)?),
                    row_registry[0].try_get::<'_, usize, String>(1)?,
                    row_registry[0].try_get::<'_, usize, String>(2)?,
                    row_registry[0].try_get::<'_, usize, String>(3)?,
                    row_registry[0].try_get::<'_, usize, String>(4)?
                )
            ));
        }

        return Ok(None);
    }
}