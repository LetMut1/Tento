use super::postgresql_repository::PostgresqlRepository;
use crate::domain_layer::data::entity::channel::Channel_Id;
use crate::domain_layer::data::entity::channel_inner_link::ChannelInnerLink;
use crate::domain_layer::data::entity::channel_inner_link::ChannelInnerLink_CreatedAt;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::types::Type;
use extern_crate::tokio_postgres::Client as Connection;

#[cfg(feature = "manual_testing")]
use extern_crate::serde::Deserialize;

impl PostgresqlRepository<ChannelInnerLink> {
    pub async fn create<'a>(
        database_1_connection: &'a Connection,
        insert: Insert,
    ) -> Result<ChannelInnerLink, ErrorAuditor> {
        let channel_inner_link_from = insert.channel_inner_link_from.get();

        let channel_inner_link_to = insert.channel_inner_link_to.get();

        let mut prepared_statemant_parameter_convertation_resolver =
            PreparedStatementParameterConvertationResolver::new();

        let query = "INSERT INTO public.channel_inner_link AS cil ( \
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
            .add_parameter(
                &channel_inner_link_from,
                Type::INT8,
            )
            .add_parameter(
                &channel_inner_link_to,
                Type::INT8,
            );

        let statement = match database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let row_registry = match database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_inner_link_created_at = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(channel_inner_link_created_at_) => ChannelInnerLink_CreatedAt::new(channel_inner_link_created_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_inner_link = ChannelInnerLink::new(
            insert.channel_inner_link_from,
            insert.channel_inner_link_to,
            channel_inner_link_created_at,
        );

        return Ok(channel_inner_link);
    }

    pub async fn find_1<'a>(
        database_1_connection: &'a Connection,
        channel_inner_link_from: Channel_Id,
        limit: i16,
    ) -> Result<Vec<ChannelInnerLink1>, ErrorAuditor> {
        let channel_inner_link_from_ = channel_inner_link_from.get();

        let mut prepared_statemant_parameter_convertation_resolver =
            PreparedStatementParameterConvertationResolver::new();

        let query = "SELECT \
                cil.to_ AS t \
            FROM public.channel_inner_link cil \
            WHERE cil.from_ = $1 \
            LIMIT $2";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &channel_inner_link_from_,
                Type::INT8,
            )
            .add_parameter(
                &limit,
                Type::INT2,
            );

        let statement = match database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let row_registry = match database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let mut channel_inner_link_registry: Vec<ChannelInnerLink1> = vec![];

        if row_registry.is_empty() {
            return Ok(channel_inner_link_registry);
        }

        '_a: for row in row_registry.iter() {
            let channel_inner_link_to = match row.try_get::<'_, usize, i64>(0) {
                Ok(channel_inner_link_to_) => Channel_Id::new(channel_inner_link_to_),
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError {
                                runtime_error: RuntimeError::ResourceError {
                                    resource_error: ResourceError::PostgresqlError {
                                        postgresql_error: error,
                                    },
                                },
                            },
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        ),
                    );
                }
            };

            let channel_inner_link = ChannelInnerLink1 {
                channel_inner_link_to,
            };

            channel_inner_link_registry.push(channel_inner_link);
        }

        return Ok(channel_inner_link_registry);
    }
}

pub struct Insert {
    pub channel_inner_link_from: Channel_Id,
    pub channel_inner_link_to: Channel_Id,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct ChannelInnerLink1 {
    pub channel_inner_link_to: Channel_Id,
}
