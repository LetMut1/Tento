use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Existing {
    pub id: Uuid,
    pub application_user_email: String
}