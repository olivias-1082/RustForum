table! {
    replies (id) {
        id -> Int4,
        reply -> Varchar,
        post_id -> Int4,
        user_id -> Int4,
        parent_comment_id -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}
table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}
table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        content -> Nullable<Varchar>,
        author -> Int4,
        created_at -> Timestamp,
    }
}
joinable!(posts -> users (author));
