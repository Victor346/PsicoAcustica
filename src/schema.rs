table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        author -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}
