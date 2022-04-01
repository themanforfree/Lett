table! {
    articles (aid) {
        aid -> Unsigned<Integer>,
        title -> Nullable<Varchar>,
        content -> Nullable<Text>,
        created -> Timestamp,
        modified -> Timestamp,
        author_id -> Nullable<Unsigned<Integer>>,
        published -> Bool,
        comments_num -> Integer,
    }
}

table! {
    comments (cid) {
        cid -> Unsigned<Integer>,
        aid -> Nullable<Unsigned<Integer>>,
        created -> Timestamp,
        author_id -> Unsigned<Integer>,
        owner_id -> Nullable<Unsigned<Integer>>,
        text -> Varchar,
    }
}

table! {
    settings (name) {
        name -> Varchar,
        value -> Nullable<Varchar>,
    }
}

table! {
    users (uid) {
        uid -> Unsigned<Integer>,
        username -> Varchar,
        password -> Varchar,
        created -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    articles,
    comments,
    settings,
    users,
);
