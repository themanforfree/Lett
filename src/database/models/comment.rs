#[derive(Queryable)]
pub struct Comment {
    pub coid: i32,
    pub cid: Option<i32>,
    pub created: Option<i32>,
    pub author_id: Option<i32>,
    pub owner_id: Option<i32>,
    pub text: Option<String>,
}
