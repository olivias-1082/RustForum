table! {
    replies (id) {
        id -> Int4,
        reply -> Varchar,
        post_id -> Int4,
        user_id -> Int4,
        parent_comment_id -> Nullable<Int4>,
        creation_time -> Timestamp,
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
        creation_time -> Timestamp,
    }
}
joinable!(posts -> users (author));
