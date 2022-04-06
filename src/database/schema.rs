table! {
    articles (aid) {
        aid -> Unsigned<Integer>,
        title -> Varchar,
        content -> Text,
        created -> Bigint,
        published -> Bool,
        comments_num -> Integer,
    }
}

table! {
    sessions (sid) {
        sid -> Varchar,
        data -> Nullable<Text>,
        expiration -> Bigint,
    }
}

allow_tables_to_appear_in_same_query!(articles, sessions,);
