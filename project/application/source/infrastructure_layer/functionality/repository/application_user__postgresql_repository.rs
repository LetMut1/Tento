use crate::domain_layer::data::entity::application_user::ApplicationUser_1;
use crate::domain_layer::data::entity::application_user::ApplicationUser_2;
use crate::domain_layer::data::entity::application_user::ApplicationUser_3;
use crate::domain_layer::data::entity::application_user::ApplicationUser_4;
use crate::domain_layer::data::entity::application_user::ApplicationUser_5;
use crate::domain_layer::data::entity::application_user::ApplicationUser_CreatedAt;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;
use crate::domain_layer::data::entity::application_user::ApplicationUser_PasswordHash;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::functionality::service::getter::Getter;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;
use std::borrow::Cow;
use super::postgresql_repository::PostgresqlRepository;

impl PostgresqlRepository<ApplicationUser<'_>> {
    pub async fn create<'a>(database_1_connection: &'a Connection, insert: Insert) -> Result<ApplicationUser<'static>, ErrorAuditor> {
        let application_user_email = insert.application_user_email.get();

        let application_user_nickname = insert.application_user_nickname.get();

        let application_user_password_hash = insert.application_user_password_hash.get();

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
                current_timestamp(6) \
            ) \
            RETURNING \
                au.id AS i,
                au.created_at::TEXT AS ca;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_email, Type::TEXT)
            .add_parameter(&application_user_nickname, Type::TEXT)
            .add_parameter(&application_user_password_hash, Type::TEXT);

        let statement = match database_1_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(application_user_id_) => ApplicationUser_Id::new(application_user_id_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_created_at = match row_registry[0].try_get::<'_, usize, String>(1) {
            Ok(application_user_created_at_) => ApplicationUser_CreatedAt::new(application_user_created_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user = ApplicationUser::new(
            application_user_id,
            Cow::Owned(insert.application_user_email),
            Cow::Owned(insert.application_user_nickname),
            insert.application_user_password_hash,
            application_user_created_at
        );

        return Ok(application_user);
    }

    pub async fn is_exist_1<'a>(database_1_connection: &'a Connection, application_user_nickname: &'a ApplicationUser_Nickname) -> Result<bool, ErrorAuditor> {
        let application_user_nickname_ = application_user_nickname.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.nickname = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_nickname_, Type::TEXT);

        let statement = match database_1_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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

    pub async fn is_exist_2<'a>(database_1_connection: &'a Connection, application_user_email: &'a ApplicationUser_Email) -> Result<bool, ErrorAuditor> {
        let application_user_email_ = application_user_email.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_email_, Type::TEXT);

        let statement = match database_1_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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

    pub async fn is_exist_3<'a>(database_1_connection: &'a Connection, application_user_id: ApplicationUser_Id) -> Result<bool, ErrorAuditor> {
        let application_user_id_ = application_user_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id_, Type::INT8);

        let statement = match database_1_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
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

    pub async fn find_1<'a>(
        database_1_connection: &'a Connection,
        application_user_nickname: &'a ApplicationUser_Nickname
    ) -> Result<Option<ApplicationUser<'a>>, ErrorAuditor> {
        let application_user_nickname_ = application_user_nickname.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.id AS i, \
                au.email AS e, \
                au.password_hash AS ph, \
                au.created_at::TEXT AS ca \
            FROM public.application_user au \
            WHERE au.nickname = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_nickname_, Type::TEXT);

        let statement = match database_1_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(application_user_id_) => ApplicationUser_Id::new(application_user_id_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_email = match row_registry[0].try_get::<'_, usize, String>(1) {
            Ok(application_user_email_) => ApplicationUser_Email::new(application_user_email_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_password_hash = match row_registry[0].try_get::<'_, usize, String>(2) {
            Ok(application_user_password_hash_) => ApplicationUser_PasswordHash::new(application_user_password_hash_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_created_at = match row_registry[0].try_get::<'_, usize, String>(3) {
            Ok(application_user_created_at_) => ApplicationUser_CreatedAt::new(application_user_created_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                ApplicationUser::new(
                    application_user_id,
                    Cow::Owned(application_user_email),
                    Cow::Borrowed(application_user_nickname),
                    application_user_password_hash,
                    application_user_created_at,
                )
            )
        );
    }
}

impl PostgresqlRepository<ApplicationUser_1> {
    pub async fn find_1<'a>(
        database_1_connection: &'a Connection,
        application_user_nickname: &'a ApplicationUser_Nickname
    ) -> Result<Option<ApplicationUser_1>, ErrorAuditor> {
        let application_user_nickname_ = application_user_nickname.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.id AS i, \
                au.email AS e, \
                au.password_hash AS ph \
            FROM public.application_user au \
            WHERE au.nickname = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_nickname_, Type::TEXT);

        let statement = match database_1_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(application_user_id_) => ApplicationUser_Id::new(application_user_id_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_email = match row_registry[0].try_get::<'_, usize, String>(1) {
            Ok(application_user_email_) => ApplicationUser_Email::new(application_user_email_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_password_hash = match row_registry[0].try_get::<'_, usize, String>(2) {
            Ok(application_user_password_hash_) => ApplicationUser_PasswordHash::new(application_user_password_hash_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                ApplicationUser_1::new(
                    application_user_id,
                    application_user_email,
                    application_user_password_hash
                )
            )
        );
    }
}

impl PostgresqlRepository<ApplicationUser_2> {
    pub async fn find_2<'a>(
        database_1_connection: &'a Connection,
        application_user_email: &'a ApplicationUser_Email
    ) -> Result<Option<ApplicationUser_2>, ErrorAuditor> {
        let application_user_email_ = application_user_email.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.id AS i, \
                au.password_hash AS ph \
            FROM public.application_user au \
            WHERE au.email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_email_, Type::TEXT);

        let statement = match database_1_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(application_user_id_) => ApplicationUser_Id::new(application_user_id_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_password_hash = match row_registry[0].try_get::<'_, usize, String>(1) {
            Ok(application_user_password_hash_) => ApplicationUser_PasswordHash::new(application_user_password_hash_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                ApplicationUser_2::new(
                    application_user_id,
                    application_user_password_hash
                )
            )
        );
    }
}

impl PostgresqlRepository<ApplicationUser_3> {
    pub async fn find_2<'a>(
        database_1_connection: &'a Connection,
        application_user_email: &'a ApplicationUser_Email
    ) -> Result<Option<ApplicationUser_3>, ErrorAuditor> {
        let application_user_email_ = application_user_email.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_email_, Type::TEXT);

        let statement = match database_1_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_id = match row_registry[0].try_get::<'_, usize, i64>(0) {
            Ok(application_user_id_) => ApplicationUser_Id::new(application_user_id_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                ApplicationUser_3::new(
                    application_user_id
                )
            )
        );
    }
}

impl PostgresqlRepository<ApplicationUser_4> {
    pub async fn update<'a, T>(
        database_1_connection: &'a Connection,
        subject: &'a T,
        application_user_id: ApplicationUser_Id
    ) -> Result<(), ErrorAuditor>
    where
        T: Getter<'a, &'a ApplicationUser_PasswordHash>
    {
        let application_user_id_ = application_user_id.get();

        let application_user_password_hash = <T as Getter<'a, &'a ApplicationUser_PasswordHash>>::get(subject).get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "UPDATE ONLY public.application_user AS au \
            SET ( \
                password_hash \
            ) = ROW( \
                $1 \
            ) \
            WHERE au.id = $2 \
            RETURNING \
                au.id AS i;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_password_hash, Type::TEXT)
            .add_parameter(&application_user_id_, Type::INT8);

        let statement = match database_1_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(error) = database_1_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        return Ok(());
    }

    pub async fn find_3<'a>(
        database_1_connection: &'a Connection,
        application_user_id: ApplicationUser_Id
    ) -> Result<Option<ApplicationUser_4>, ErrorAuditor> {
        let application_user_id_ = application_user_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.password_hash AS ph \
            FROM public.application_user au \
            WHERE au.id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id_, Type::INT8);

        let statement = match database_1_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_password_hash = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_password_hash_) => ApplicationUser_PasswordHash::new(application_user_password_hash_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                ApplicationUser_4::new(application_user_password_hash)
            )
        );
    }
}

impl PostgresqlRepository<ApplicationUser_5> {
    pub async fn find_3<'a>(
        database_1_connection: &'a Connection,
        application_user_id: ApplicationUser_Id
    ) -> Result<Option<ApplicationUser_5>, ErrorAuditor> {
        let application_user_id_ = application_user_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                au.email AS e \
            FROM public.application_user au \
            WHERE au.id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&application_user_id_, Type::INT8);

        let statement = match database_1_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_1_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_email = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_email_) => ApplicationUser_Email::new(application_user_email_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                ApplicationUser_5::new(
                    application_user_email
                )
            )
        );
    }
}

pub struct Insert {
    pub application_user_email: ApplicationUser_Email,
    pub application_user_nickname: ApplicationUser_Nickname,
    pub application_user_password_hash: ApplicationUser_PasswordHash
}