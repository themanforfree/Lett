table! {
    articles (aid) {
        aid -> Unsigned<Integer>,
        title -> Varchar,
        content -> Text,
        created -> Timestamp,
        published -> Bool,
        comments_num -> Integer,
    }
}
