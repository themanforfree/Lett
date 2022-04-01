table! {
    comments (coid) {
        coid -> Unsigned<Integer>,
        cid -> Nullable<Unsigned<Integer>>,
        created -> Datetime,
        authorId -> Unsigned<Integer>,
        ownerId -> Nullable<Unsigned<Integer>>,
        text -> Varchar,
    }
}

table! {
    contents (cid) {
        cid -> Unsigned<Integer>,
        title -> Nullable<Varchar>,
        created -> Datetime,
        modified -> Datetime,
        authorId -> Nullable<Unsigned<Integer>>,
        published -> Nullable<Varchar>,
        commentsNum -> Nullable<Integer>,
    }
}

table! {
    setting (name) {
        name -> Varchar,
        value -> Nullable<Varchar>,
    }
}

table! {
    users (uid) {
        uid -> Unsigned<Integer>,
        username -> Varchar,
        password -> Varchar,
        created -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    comments,
    contents,
    setting,
    users,
);
