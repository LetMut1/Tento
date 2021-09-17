use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as StateManagerApplicationUserPostgresqlTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::_new_for_context::update::_new_for_context::update_resolver::UpdateResolver;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use postgres::Client as Connection;
use postgres::Row;
use postgres::Statement;
use postgres::types::Type;

pub struct Base;

impl StateManagerApplicationUserPostgresqlTrait for Base {
    type Error = BaseError;

    fn create<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user: &'outer_a ApplicationUser<'_>
    ) -> Result<(), Self::Error> {
        let email: &'_ str = application_user.get_email();

        let nickanme: &'_ str = application_user.get_nickname();

        let password_hash: &'_ str = application_user.get_password_hash();

        let created_at: &'_ str = application_user.get_created_at();

        let mut prepared_statemant_parameter_convertation_resolver: PreparedStatementParameterConvertationResolver<'_> = PreparedStatementParameterConvertationResolver::new();

        let query: &'static str = 
            "INSERT INTO public.application_user AS au ( \
                id, \
                email, \
                nickname, \
                password_hash, \
                created_at \
            ) VALUES ( \
                nextval('public.application_user_pre_confirmed1'), \
                $1, \
                $2, \
                $3, \
                $4::TIMESTAMP(6) WITH TIME ZONE \
            ) \
            ON CONFLICT DO NOTHING \
            RETURNING \
                au.id AS i;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&email, Type::TEXT)
            .add_parameter(&nickanme, Type::TEXT)
            .add_parameter(&password_hash, Type::TEXT)
            .add_parameter(&created_at, Type::TEXT);

        let statement: Statement = connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry())?;

        let row_registry: Vec<Row> = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry())?;
        if row_registry.is_empty() {
            return Err(BaseError::LogicError {message: "ApplicationUser can not be inserted into Postgesql database."});
        }

        return Ok(());
    }

    fn update<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user: &'outer_a ApplicationUser<'_>,
        update_resolver: UpdateResolver                     // TODO изменить нэймспейс !!!!!! НАПИСАТЬ проверку на отсутсвия ТРУ для полей обновления.
    ) -> Result<(), Self::Error> {

        // TODO Переписать здесь все после введения нового АпдейтРезолвера
        //  https://postgrespro.ru/docs/postgresql/9.6/sql-update

        // let email: &'_ str = application_user.get_email();

        // let nickanme: &'_ str = application_user.get_nickname();

        // let password_hash: &'_ str = application_user.get_password_hash();

        // let mut prepared_statemant_parameter_convertation_resolver: PreparedStatementParameterConvertationResolver<'_> = PreparedStatementParameterConvertationResolver::new();

        // let query: String = 
        //     "UPDATE ONLY public.application_user AS au \
        //     SET ( \
        //         email, \
        //         nickname, \
        //         password_hash \
        //     ) = ( \
        //         $1, \
        //         $2, \
        //         $3 \
        //     ) \
        //     WHERE au.id = $4 \
        //     RETURNING \
        //         au.id AS i;"
        //     .to_string();










        // prepared_statemant_parameter_convertation_resolver
        //     .add_parameter(&email, Type::TEXT)
        //     .add_parameter(&nickanme, Type::TEXT)
        //     .add_parameter(&password_hash, Type::TEXT);

        // let statement: Statement = connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry())?;

        // let row_registry: Vec<Row> = connection.query(&statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry())?;
        // if row_registry.is_empty() {
        //     return Err(BaseError::LogicError {message: "ApplicationUser can not be inserted into Postgesql database."});
        // }
        return Ok(());
    }
}