#[derive(Queryable)]
pub struct User {
    pub uid: i32,
    pub username: String,
    pub password: String,
    pub created: i32,
}
