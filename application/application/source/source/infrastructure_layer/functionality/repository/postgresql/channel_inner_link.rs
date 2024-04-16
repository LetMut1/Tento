use super::by::By8;
use super::insert::Insert8;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::channel::Channel_Id;
use crate::domain_layer::data::entity::channel_inner_link::ChannelInnerLink;
use crate::domain_layer::data::entity::channel_inner_link::ChannelInnerLink_CreatedAt;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::Auditor;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use tokio_postgres::types::Type;
use crate::infrastructure_layer::data::error_auditor::Other;
use tokio_postgres::Client as Connection;
pub use action_processor_incoming_outcoming::ChannelInnerLink1;

impl PostgresqlRepository<ChannelInnerLink> {
    pub async fn create<'a>(
        database_1_connection: &'a Connection,
        insert_8: Insert8,
    ) -> Result<ChannelInnerLink, Auditor<Error>> {
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
            .add_parameter(
                &insert_8.channel_inner_link_from.0,
                Type::INT8,
            )
            .add_parameter(
                &insert_8.channel_inner_link_to.0,
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

        let channel_inner_link_created_at = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(channel_inner_link_created_at_) => ChannelInnerLink_CreatedAt(channel_inner_link_created_at_),
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
            ChannelInnerLink {
                from: insert_8.channel_inner_link_from,
                to: insert_8.channel_inner_link_to,
                created_at: channel_inner_link_created_at,
            },
        );
    }

    pub async fn find_1<'a>(
        database_1_connection: &'a Connection,
        by_8: &'a By8,
        limit: i16,
    ) -> Result<Vec<ChannelInnerLink1>, Auditor<Error>> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                cil.to_ AS t \
            FROM public.channel_inner_link cil \
            WHERE cil.from_ = $1 \
            LIMIT $2";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_8.channel_inner_link_from.0,
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

        let mut channel_inner_link_registry: Vec<ChannelInnerLink1> = vec![];

        if row_registry.is_empty() {
            return Ok(channel_inner_link_registry);
        }

        '_a: for row in row_registry.iter() {
            let channel_inner_link_to = match row.try_get::<'_, usize, i64>(0) {
                Ok(channel_inner_link_to_) => Channel_Id(channel_inner_link_to_),
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

            let channel_inner_link = ChannelInnerLink1 {
                channel_inner_link_to,
            };

            channel_inner_link_registry.push(channel_inner_link);
        }

        return Ok(channel_inner_link_registry);
    }
}
