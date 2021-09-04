use postgres::types::ToSql;
use postgres::types::Type;

pub struct PreparedStatementParameterConvertationResolver<'outer_a> {
    parameter_registry: Vec<&'outer_a (dyn ToSql + Sync)>,
    parameter_type_registry: Vec<Type>
}

impl<'outer_a> PreparedStatementParameterConvertationResolver<'outer_a> {
    pub fn new() -> Self {
        return Self {
            parameter_registry: Vec::new(),
            parameter_type_registry: Vec::new()
        };
    }

    pub fn add_parameter<'this>(&'this mut self, parameter_value: &'outer_a (dyn ToSql + Sync), patameter_type: Type) -> &'this mut Self {
        self.parameter_registry.push(parameter_value);
        self.parameter_type_registry.push(patameter_type);

        return self;
    }

    pub fn get_parameter_registry<'this>(&'this self) -> &'this Vec<&'outer_a (dyn ToSql + Sync)> {
        return &self.parameter_registry;
    }

    pub fn get_parameter_type_registry<'this>(&'this self) -> &'this Vec<Type> {
        return &self.parameter_type_registry;
    }
}