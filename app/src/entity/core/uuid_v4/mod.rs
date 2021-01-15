use diesel::AsExpression;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::deserialize::Result as DieselDeserializeResult;
use diesel::FromSqlRow;
use diesel::serialize::Output;
use diesel::serialize::Result as DieselSerializeResult;
use diesel::serialize::ToSql;
use diesel::sql_types::Uuid as DieselSqlTypeUuid;
use std::fmt::Debug;
use std::io::Write;
use uuid::Uuid;

#[derive(Debug, AsExpression, FromSqlRow)]
#[sql_type = "DieselSqlTypeUuid"]
pub struct UuidV4 {
    value: Uuid
}

impl<'a> UuidV4 {
    pub fn new() -> Self {
        return Self {value: Uuid::new_v4()};
    }

    pub fn set_value(&'a mut self, value: Uuid) -> &'a mut Self {
        self.value = value;

        return self;
    }

    pub fn get_value(&'a mut self) -> &'a mut Uuid {
        return &mut self.value;
    }
}

impl<DB> ToSql<DieselSqlTypeUuid, DB> for UuidV4
where
    DB: Backend,
    Uuid: ToSql<DieselSqlTypeUuid, DB>
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> DieselSerializeResult 
    where 
        W: Write
    {
        return self.value.to_sql(out);    
    }
}

impl<DB> FromSql<DieselSqlTypeUuid, DB> for UuidV4
where
     DB: Backend,
     Uuid: FromSql<DieselSqlTypeUuid, DB>,
 {
     fn from_sql(bytes: Option<&DB::RawValue>) -> DieselDeserializeResult<Self> {
         return Ok(Self {value: Uuid::from_sql(bytes)?});
     }
}