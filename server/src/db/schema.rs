pub mod gib_pm {
    table! {
        gib_pm.package_archives (id) {
            id -> Int4,
            package_id -> Int4,
            version -> Text,
            archive -> Bytea,
        }
    }

    table! {
        gib_pm.packages (id) {
            id -> Int4,
            package_name -> Text,
            publisher -> Int4,
            configuration -> Text,
            current_version -> Text,
        }
    }

    table! {
        gib_pm.users (id) {
            id -> Int4,
            username -> Text,
            password -> Varchar,
            email -> Text,
        }
    }

    allow_tables_to_appear_in_same_query!(
        package_archives,
        packages,
        users,
    );
}
