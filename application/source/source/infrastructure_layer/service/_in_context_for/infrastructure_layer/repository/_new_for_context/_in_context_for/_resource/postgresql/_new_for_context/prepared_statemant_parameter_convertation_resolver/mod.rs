use postgres::types::ToSql;
use postgres::types::Type;

pub struct PreparedStatementParameterConvertationResolver<'a> {
    parameter_registry: Vec<&'a (dyn ToSql + Sync)>,
    parameter_type_registry: Vec<Type>
}

impl<'a> PreparedStatementParameterConvertationResolver<'a> {
    pub fn new(
    ) -> Self {
        return Self {
            parameter_registry: Vec::new(),
            parameter_type_registry: Vec::new()
        };
    }

    pub fn add_parameter<'b>(
        &'b mut self,
        parameter_value: &'a (dyn ToSql + Sync),
        patameter_type: Type
    ) -> &'b mut Self {
        self.parameter_registry.push(parameter_value);
        self.parameter_type_registry.push(patameter_type);

        return self;
    }

    pub fn get_parameter_registry<'b>(
        &'b self
    ) -> &'b Vec<&'a (dyn ToSql + Sync)> {
        return &self.parameter_registry;
    }

    pub fn get_parameter_type_registry<'b>(
        &'b self
    ) -> &'b Vec<Type> {
        return &self.parameter_type_registry;
    }
}