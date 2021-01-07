pub mod public {
    table! {
        use diesel::sql_types::*;
        
        app_user (id) {
            email -> Varchar,
            id -> Uuid,
            jwt_id -> Uuid,
            nickname -> Varchar,
        }
    }
}
