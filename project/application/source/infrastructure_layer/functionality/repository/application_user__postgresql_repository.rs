use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::LogicError;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;

#[allow(non_camel_case_types)]
pub struct ApplicationUser_PostgresqlRepository;

impl ApplicationUser_PostgresqlRepository {
    pub async fn create<'a>(database_1_connection: &'a Connection, insert: Insert) -> Result<ApplicationUser, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "INSERT INTO public.application_user AS au ( \
                id, \
                email, \
                nickname, \
                password_hash, \
                created_at \
            ) VALUES ( \
                nextval('public.application_user1'), \
                $1, \
                $2, \
                $3, \
                DEFAULT \
            ) \
            RETURNING \
                au.id AS i,
                au.created_at::TEXT AS ca;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&insert.application_user_email, Type::TEXT)
            .add_parameter(&insert.application_user_nickname, Type::TEXT)
            .add_parameter(&insert.application_user_password_hash, Type::TEXT);

        let statement = match database_1_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Err(
                ErrorAuditor::new(
                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUser can not be inserted into Postgresql database.") },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        let application_user_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(application_user_id_) => application_user_id_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_created_at = match row_registry[0].try_get::<'_, usize, String>(1) {
            Ok(application_user_created_at_) => application_user_created_at_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user = ApplicationUser::new(
            application_user_id,
            insert.application_user_email,
            insert.application_user_nickname,
            insert.application_user_password_hash,
            application_user_created_at
        );

        return Ok(application_user);
    }

    pub async fn update<'a>(database_1_connection: &'a Connection, application_user: &'a ApplicationUser) -> Result<(), ErrorAuditor> {
        let application_user_id = application_user.get_id();

        let application_user_email = application_user.get_email();

        let application_user_nickname = application_user.get_nickname();

        let application_user_password_hash = application_user.get_password_hash();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "UPDATE ONLY public.application_user AS au \
            SET ( \
                email, \
                nickname, \
                password_hash \
            ) = ROW( \
                $1, \
                $2, \
                $3 \
            ) \
            WHERE au.id = $4 \
            RETURNING \
                au.id AS i;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_email, Type::TEXT)
            .add_parameter(&application_user_nickname, Type::TEXT)
            .add_parameter(&application_user_password_hash, Type::TEXT)
            .add_parameter(&application_user_id, Type::INT8);

        let statement = match database_1_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Err(
                ErrorAuditor::new(
                    BaseError::LogicError { logic_error: LogicError::new(false, "ApplicationUser can not be updated in Postgresql database.") },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        return Ok(());
    }

    pub async fn is_exist_1<'a>(database_1_connection: &'a Connection, application_user_nickname: &'a str) -> Result<bool, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.nickname = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_nickname, Type::TEXT);

        let statement = match database_1_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(false);
        }

        return Ok(true);
    }

    pub async fn is_exist_2<'a>(database_1_connection: &'a Connection, application_user_email: &'a str) -> Result<bool, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_email, Type::TEXT);

        let statement = match database_1_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(false);
        }

        return Ok(true);
    }

    pub async fn is_exist_3<'a>(database_1_connection: &'a Connection, application_user_id: i64) -> Result<bool, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id, Type::INT8);

        let statement = match database_1_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(false);
        }

        return Ok(true);
    }

    pub async fn find_1<'a>(database_1_connection: &'a Connection, application_user_nickname: String) -> Result<Option<ApplicationUser>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.id AS i, \
                au.email AS e, \
                au.password_hash AS ph, \
                au.created_at::TEXT AS ca \
            FROM public.application_user au \
            WHERE au.nickname = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_nickname, Type::TEXT);

        let statement = match database_1_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(application_user_id_) => application_user_id_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_email = match row_registry[0].try_get::<'_, usize, String>(1) {
            Ok(application_user_email_) => application_user_email_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_password_hash = match row_registry[0].try_get::<'_, usize, String>(2) {
            Ok(application_user_password_hash_) => application_user_password_hash_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_created_at = match row_registry[0].try_get::<'_, usize, String>(3) {
            Ok(application_user_created_at_) => application_user_created_at_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                ApplicationUser::new(
                    application_user_id,
                    application_user_email,
                    application_user_nickname,
                    application_user_password_hash,
                    application_user_created_at,
                )
            )
        );
    }

    pub async fn find_2<'a>(database_1_connection: &'a Connection, application_user_email: String) -> Result<Option<ApplicationUser>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.id AS i, \
                au.nickname AS n, \
                au.password_hash AS ph, \
                au.created_at::TEXT AS ca \
            FROM public.application_user au \
            WHERE au.email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_email, Type::TEXT);

        let statement = match database_1_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(application_user_id_) => application_user_id_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_nickname = match row_registry[0].try_get::<'_, usize, String>(1) {
            Ok(application_user_nickname_) => application_user_nickname_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_password_hash = match row_registry[0].try_get::<'_, usize, String>(2) {
            Ok(application_user_password_hash_) => application_user_password_hash_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_created_at = match row_registry[0].try_get::<'_, usize, String>(3) {
            Ok(application_user_created_at_) => application_user_created_at_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                ApplicationUser::new(
                    application_user_id,
                    application_user_email,
                    application_user_nickname,
                    application_user_password_hash,
                    application_user_created_at,
                )
            )
        );
    }

    pub async fn find_3<'a>(database_1_connection: &'a Connection, application_user_id: i64) -> Result<Option<ApplicationUser>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.email AS e, \
                au.nickname AS n, \
                au.password_hash AS ph, \
                au.created_at::TEXT AS ca \
            FROM public.application_user au \
            WHERE au.id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id, Type::INT8);

        let statement = match database_1_connection.prepare_typed(
            query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_email = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_email_) => application_user_email_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_nickname = match row_registry[0].try_get::<'_, usize, String>(1) {
            Ok(application_user_nickname_) => application_user_nickname_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_password_hash = match row_registry[0].try_get::<'_, usize, String>(2) {
            Ok(application_user_password_hash_) => application_user_password_hash_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_created_at = match row_registry[0].try_get::<'_, usize, String>(3) {
            Ok(application_user_created_at_) => application_user_created_at_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                ApplicationUser::new(
                    application_user_id,
                    application_user_email,
                    application_user_nickname,
                    application_user_password_hash,
                    application_user_created_at,
                )
            )
        );
    }
}

pub struct Insert {
    pub application_user_email: String,
    pub application_user_nickname: String,
    pub application_user_password_hash: String
}