#[derive(Queryable)]
pub struct Setting {
    pub name: String,
    pub value: Option<String>,
}
