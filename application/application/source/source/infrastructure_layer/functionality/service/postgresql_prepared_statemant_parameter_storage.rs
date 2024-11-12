use tokio_postgres::types::{
    ToSql,
    Type,
};
pub struct PostgresqlPreparedStatementParameterStorage<'a, 'b> {
    parameter_registry: Vec<&'a (dyn ToSql + Sync + 'b)>,
    parameter_type_registry: Vec<Type>,
}
impl<'a, 'b> PostgresqlPreparedStatementParameterStorage<'a, 'b> {
    pub fn new() -> Self {
        return Self {
            parameter_registry: vec![],
            parameter_type_registry: vec![],
        };
    }
    pub fn add<'c>(&'c mut self, parameter_value: &'a (dyn ToSql + Sync + 'b), patameter_type: Type) -> &'c mut Self {
        self.parameter_registry.push(parameter_value);
        self.parameter_type_registry.push(patameter_type);
        return self;
    }
    pub fn get_parameter_registry<'c>(&'c self) -> &'c [&'a (dyn ToSql + Sync + 'b)] {
        return &self.parameter_registry;
    }
    pub fn get_parameter_type_registry<'c>(&'c self) -> &'c [Type] {
        return &self.parameter_type_registry;
    }
}
