
extern crate chrono;
use uuid::v1::{Timestamp, Context};
use uuid::Uuid;
// TODO delete


use diesel::AsExpression;
use diesel::backend::Backend;
use diesel::serialize::Output;
use diesel::serialize::Result as DieselSerializeResult;
use diesel::serialize::ToSql;
use diesel::sql_types::Text as DieselSqlTypeText;
use diesel::sql_types::Uuid as DieselSqlTypeUuid;
use std::fmt::Debug;
use std::io::Write;

#[derive(Debug, AsExpression)]
#[sql_type = "DieselSqlTypeUuid"]
pub struct Id {
    value: String
}

impl<'a> Id {
    pub fn new() -> Self {

        let val = chrono::prelude::Utc::now();
            //TODO проверить в фикстурах, будет ли последовательно, или нет. Если да, вынести в функцию, если нет, использовать здесь метода v4
        let context = Context::new(42);
        let ts = Timestamp::from_unix(&context, val.timestamp() as u64, val.timestamp_subsec_millis());
        let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6]).expect("failed to generate UUID"); 
        //TODO

        return Self {value: uuid.to_string()};
    }

    pub fn set_value(&'a mut self, value: String) -> &'a mut Self {
        self.value = value;

        return self;
    }

    pub fn get_value(&'a self) -> String {
        return self.value.clone();
    }
}

impl<DB> ToSql<DieselSqlTypeUuid, DB> for Id
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
