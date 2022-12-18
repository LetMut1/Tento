use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::_in_context_for::infrastructure_layer::functionality::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;

pub struct Base;

impl Base {
    pub async fn create<'a>(
        core_connection: &'a Connection,
        insert: Insert
    ) -> Result<ApplicationUser, ErrorAuditor> {
        let (
            application_user_email,
            application_user_nickname,
            application_user_password_hash
        ) = insert.into_inner();

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
            .add_parameter(&application_user_email, Type::TEXT)
            .add_parameter(&application_user_nickname, Type::TEXT)
            .add_parameter(&application_user_password_hash, Type::TEXT);

        match core_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match core_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
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
                            application_user_email,
                            application_user_nickname,
                            application_user_password_hash,
                            application_user_created_at
                        );

                        return Ok(application_user);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn update<'a>(
        core_connection: &'a Connection,
        application_user: &'a ApplicationUser
    ) -> Result<(), ErrorAuditor> {
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

        match core_connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry().as_slice()).await {
            Ok(ref statement) => {
                match core_connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry().as_slice()).await {
                    Ok(row_registry) => {
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
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}

pub struct Insert {
    application_user_email: String,
    application_user_nickname: String,
    application_user_password_hash: String
}

impl Insert {
    pub fn new(
        application_user_email: String,
        application_user_nickname: String,
        application_user_password_hash: String
    ) -> Self {
        return Self {
            application_user_email,
            application_user_nickname,
            application_user_password_hash
        }
    }

    pub fn into_inner(
        self
    ) -> (String, String, String) {
        return (
            self.application_user_email,
            self.application_user_nickname,
            self.application_user_password_hash
        );
    }
}