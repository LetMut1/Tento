pub mod public {
    table! {
        use diesel::sql_types::*;

        application_user (id) {
            email -> Varchar,
            id -> Uuid,
            jwt_id -> Uuid,
            nickname -> Varchar,
        }
    }

    table! {
        use diesel::sql_types::*;

        for_test (idd) {
            testes_value -> Int8,
            idd -> Uuid,
        }
    }

    allow_tables_to_appear_in_same_query!(
        application_user,
        for_test,
    );
}