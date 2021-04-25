pub mod public {
    table! {
        use diesel::sql_types::*;

        application_user (id) {
            id -> Uuid,
            email -> Varchar,
            nickname -> Varchar,
            password_hash -> Varchar,
            created_at -> Timestamptz,
        }
    }

    table! {
        use diesel::sql_types::*;

        pre_confirmed_application_user (id) {
            id -> Uuid,
            email -> Varchar,
        }
    }

    allow_tables_to_appear_in_same_query!(
        application_user,
        pre_confirmed_application_user,
    );
}
