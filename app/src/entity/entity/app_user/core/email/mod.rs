use diesel::AsExpression;
use diesel::backend::Backend;
use diesel::serialize::Output;
use diesel::serialize::Result as DieselSerializeResult;
use diesel::serialize::ToSql;
use diesel::sql_types::Text as DieselSqlTypeText;
use std::fmt::Debug;
use std::io::Write;

#[derive(Debug, AsExpression)]
#[sql_type = "DieselSqlTypeText"]
pub struct Email {
    value: String
}

impl<'a> Email {
    pub fn new(value: String) -> Self {
        return Self {value};
    }

    pub fn set_value(&'a mut self, value: String) -> &'a mut Self {
        self.value = value;

        return self;
    }

    pub fn get_value(&'a self) -> String {
        return self.value.clone();
    }
}

impl<DB> ToSql<DieselSqlTypeText, DB> for Email
where
    DB: Backend,
    String: ToSql<DieselSqlTypeText, DB>
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> DieselSerializeResult 
    where 
        W: Write
    {
        return self.get_value().to_sql(out);
    }
}


