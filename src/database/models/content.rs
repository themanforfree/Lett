#[derive(Queryable)]
pub struct Content {
    pub cid: i32,
    pub title: Option<String>,
    pub created: Option<i32>,
    pub modified: Option<i32>,
    pub author_id: Option<i32>,
    pub published: Option<String>,
    pub comments_num: Option<i32>,
}
