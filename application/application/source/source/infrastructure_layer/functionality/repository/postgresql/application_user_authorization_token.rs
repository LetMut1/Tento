use super::by::By4;
use super::insert::Insert3;
use super::update::Update3;
use super::update::Update4;
use super::update::Update5;
use super::update::Update6;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken1;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken2;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken3;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken4;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken5;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use std::borrow::Cow;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;

impl PostgresqlRepository<ApplicationUserAuthorizationToken<'_>> {
    pub async fn create<'a>(
        database_2_connection: &'a Connection,
        insert_3: Insert3<'a>,
    ) -> Result<ApplicationUserAuthorizationToken<'a>, Auditor<Error>> {
        let application_user_device_id = insert_3.application_user_device_id.0.as_str();

        let application_user_authorization_token_value = insert_3.application_user_authorization_token_value.0.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            INSERT INTO public.application_user_authorization_token AS auat ( \
                application_user_id, \
                application_user_device_id, \
                value, \
                wrong_enter_tries_quantity, \
                expires_at, \
                can_be_resent_from \
            ) VALUES ( \
                $1, \
                $2, \
                $3, \
                $4, \
                $5, \
                $6 \
            );";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &insert_3.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            )
            .add_parameter(
                &application_user_authorization_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &insert_3.application_user_authorization_token_wrong_enter_tries_quantity.0,
                Type::INT2,
            )
            .add_parameter(
                &insert_3.application_user_authorization_token_expires_at.0,
                Type::INT8,
            )
            .add_parameter(
                &insert_3.application_user_authorization_token_can_be_resent_from.0,
                Type::INT8,
            );

        let statement = match database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        if let Err(error) = database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        };

        return Ok(
            ApplicationUserAuthorizationToken {
                application_user_id: insert_3.application_user_id,
                application_user_device_id: Cow::Borrowed(insert_3.application_user_device_id),
                value: insert_3.application_user_authorization_token_value,
                wrong_enter_tries_quantity: insert_3.application_user_authorization_token_wrong_enter_tries_quantity,
                expires_at: insert_3.application_user_authorization_token_expires_at,
                can_be_resent_from: insert_3.application_user_authorization_token_can_be_resent_from,
            },
        );
    }

    pub async fn delete<'a>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let application_user_device_id = by_4.application_user_device_id.0.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            DELETE FROM ONLY public.application_user_authorization_token AS auat \
            WHERE auat.application_user_id = $1 AND auat.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_4.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            );

        let statement = match database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        if let Err(error) = database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        };

        return Ok(());
    }
}

impl PostgresqlRepository<ApplicationUserAuthorizationToken1> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_3: &'a Update3<'_>,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let application_user_device_id = by_4.application_user_device_id.0.as_str();

        let application_user_authorization_token_value = update_3.application_user_authorization_token_value.0.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_authorization_token AS auat \
            SET ( \
                value, \
                wrong_enter_tries_quantity, \
                expires_at, \
                can_be_resent_from \
            ) = ROW( \
                $1, \
                $2, \
                $3, \
                $4 \
            ) \
            WHERE auat.application_user_id = $5 AND auat.application_user_device_id = $6;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_authorization_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &update_3.application_user_authorization_token_wrong_enter_tries_quantity.0,
                Type::INT2,
            )
            .add_parameter(
                &update_3.application_user_authorization_token_expires_at.0,
                Type::INT8,
            )
            .add_parameter(
                &update_3.application_user_authorization_token_can_be_resent_from.0,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            );

        let statement = match database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        if let Err(error) = database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        };

        return Ok(());
    }

    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'_>,
    ) -> Result<Option<ApplicationUserAuthorizationToken1>, Auditor<Error>> {
        let application_user_device_id = by_4.application_user_device_id.0.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                auat.value AS v, \
                auat.wrong_enter_tries_quantity AS wetq, \
                auat.expires_at AS ea, \
                auat.can_be_resent_from AS cbrf \
            FROM public.application_user_authorization_token auat \
            WHERE auat.application_user_id = $1 AND auat.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_4.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            );

        let statement = match database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let row_registry = match database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_authorization_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_authorization_token_value_) => ApplicationUserAuthorizationToken_Value(application_user_authorization_token_value_),
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let application_user_authorization_token_wrong_enter_tries_quantity = match row_registry[0].try_get::<'_, usize, i16>(1) {
            Ok(application_user_authorization_token_wrong_enter_tries_quantity_) => ApplicationUserAuthorizationToken_WrongEnterTriesQuantity(application_user_authorization_token_wrong_enter_tries_quantity_),
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let application_user_authorization_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(2) {
            Ok(application_user_authorization_token_expires_at_) => ApplicationUserAuthorizationToken_ExpiresAt(application_user_authorization_token_expires_at_),
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let application_user_authorization_token_can_be_resent_from = match row_registry[0].try_get::<'_, usize, i64>(3) {
            Ok(application_user_authorization_token_can_be_resent_from_) => ApplicationUserAuthorizationToken_CanBeResentFrom(application_user_authorization_token_can_be_resent_from_),
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        return Ok(
            Some(
                ApplicationUserAuthorizationToken1 {
                    value: application_user_authorization_token_value,
                    wrong_enter_tries_quantity: application_user_authorization_token_wrong_enter_tries_quantity,
                    expires_at: application_user_authorization_token_expires_at,
                    can_be_resent_from: application_user_authorization_token_can_be_resent_from,
                },
            ),
        );
    }
}

impl PostgresqlRepository<ApplicationUserAuthorizationToken2> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_4: &'a Update4<'_>,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let application_user_device_id = by_4.application_user_device_id.0.as_str();

        let application_user_authorization_token_value = update_4.application_user_authorization_token_value.0.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_authorization_token AS auat \
            SET ( \
                value, \
                wrong_enter_tries_quantity, \
                expires_at \
            ) = ROW( \
                $1, \
                $2, \
                $3 \
            ) \
            WHERE auat.application_user_id = $4 AND auat.application_user_device_id = $5;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &application_user_authorization_token_value,
                Type::TEXT,
            )
            .add_parameter(
                &update_4.application_user_authorization_token_wrong_enter_tries_quantity.0,
                Type::INT2,
            )
            .add_parameter(
                &update_4.application_user_authorization_token_expires_at.0,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            );

        let statement = match database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        if let Err(error) = database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        };

        return Ok(());
    }

    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'_>,
    ) -> Result<Option<ApplicationUserAuthorizationToken2>, Auditor<Error>> {
        let application_user_device_id = by_4.application_user_device_id.0.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                auat.value AS v, \
                auat.wrong_enter_tries_quantity AS wetq, \
                auat.expires_at AS ea \
            FROM public.application_user_authorization_token auat \
            WHERE auat.application_user_id = $1 AND auat.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_4.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            );

        let statement = match database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let row_registry = match database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_authorization_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_authorization_token_value_) => ApplicationUserAuthorizationToken_Value(application_user_authorization_token_value_),
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let application_user_authorization_token_wrong_enter_tries_quantity = match row_registry[0].try_get::<'_, usize, i16>(1) {
            Ok(application_user_authorization_token_wrong_enter_tries_quantity_) => ApplicationUserAuthorizationToken_WrongEnterTriesQuantity(application_user_authorization_token_wrong_enter_tries_quantity_),
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let application_user_authorization_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(2) {
            Ok(application_user_authorization_token_expires_at_) => ApplicationUserAuthorizationToken_ExpiresAt(application_user_authorization_token_expires_at_),
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        return Ok(
            Some(
                ApplicationUserAuthorizationToken2 {
                    value: application_user_authorization_token_value,
                    wrong_enter_tries_quantity: application_user_authorization_token_wrong_enter_tries_quantity,
                    expires_at: application_user_authorization_token_expires_at,
                },
            ),
        );
    }
}

impl PostgresqlRepository<ApplicationUserAuthorizationToken3> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_5: &'a Update5,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let application_user_device_id = by_4.application_user_device_id.0.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_authorization_token AS auat \
            SET ( \
                can_be_resent_from \
            ) = ROW( \
                $1 \
            ) \
            WHERE auat.application_user_id = $2 AND auat.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_5.application_user_authorization_token_can_be_resent_from.0,
                Type::INT8,
            )
            .add_parameter(
                &by_4.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            );

        let statement = match database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        if let Err(error) = database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        };

        return Ok(());
    }
}

impl PostgresqlRepository<ApplicationUserAuthorizationToken4> {
    pub async fn update<'a>(
        database_2_connection: &'a Connection,
        update_6: &'a Update6,
        by_4: &'a By4<'_>,
    ) -> Result<(), Auditor<Error>> {
        let application_user_device_id = by_4.application_user_device_id.0.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            UPDATE ONLY public.application_user_authorization_token AS auat \
            SET ( \
                wrong_enter_tries_quantity \
            ) = ROW( \
                $1 \
            ) \
            WHERE auat.application_user_id = $2 AND auat.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &update_6.application_user_authorization_token_wrong_enter_tries_quantity.0,
                Type::INT2,
            )
            .add_parameter(
                &by_4.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            );

        let statement = match database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        if let Err(error) = database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            return Err(
                Auditor::<Error>::new(
                    Error::Runtime {
                        runtime: Runtime::Other {
                            other: Other::new(error),
                        },
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        };

        return Ok(());
    }
}

impl PostgresqlRepository<ApplicationUserAuthorizationToken5> {
    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        by_4: &'a By4<'_>,
    ) -> Result<Option<ApplicationUserAuthorizationToken5>, Auditor<Error>> {
        let application_user_device_id = by_4.application_user_device_id.0.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                auat.value AS v, \
                auat.expires_at AS ea, \
                auat.can_be_resent_from AS cbrf \
            FROM public.application_user_authorization_token auat \
            WHERE auat.application_user_id = $1 AND auat.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_4.application_user_id.0,
                Type::INT8,
            )
            .add_parameter(
                &application_user_device_id,
                Type::TEXT,
            );

        let statement = match database_2_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let row_registry = match database_2_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_authorization_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_authorization_token_value_) => ApplicationUserAuthorizationToken_Value(application_user_authorization_token_value_),
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let application_user_authorization_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(1) {
            Ok(application_user_authorization_token_expires_at_) => ApplicationUserAuthorizationToken_ExpiresAt(application_user_authorization_token_expires_at_),
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let application_user_authorization_token_can_be_resent_from = match row_registry[0].try_get::<'_, usize, i64>(2) {
            Ok(application_user_authorization_token_can_be_resent_from_) => ApplicationUserAuthorizationToken_CanBeResentFrom(application_user_authorization_token_can_be_resent_from_),
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        return Ok(
            Some(
                ApplicationUserAuthorizationToken5 {
                    value: application_user_authorization_token_value,
                    expires_at: application_user_authorization_token_expires_at,
                    can_be_resent_from: application_user_authorization_token_can_be_resent_from,
                },
            ),
        );
    }
}
