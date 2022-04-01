use super::Crud;
use crate::database::schema::settings;
use diesel::{prelude::*, result::Error, MysqlConnection};
#[derive(Queryable, AsChangeset)]
pub struct Setting {
    pub name: String,
    pub value: Option<String>,
}

impl Crud<Setting, String> for Setting {
    fn create(_conn: &MysqlConnection, _from: &Setting) -> Result<Self, Error> {
        unimplemented!()
    }
    fn read(_conn: &MysqlConnection) -> Vec<Self> {
        unimplemented!()
    }
    fn update(conn: &MysqlConnection, pk: String, value: &Setting) -> Result<Self, Error> {
        diesel::update(settings::table.find(&pk))
            .set(value)
            .execute(conn)?;
        settings::table.find(pk).first::<Self>(conn)
    }
    fn delete(_conn: &MysqlConnection, _pk: String) -> Result<usize, Error> {
        unimplemented!()
    }
    fn get_by_pk(_conn: &MysqlConnection, _pk: String) -> Result<Self, Error> {
        unimplemented!()
    }
}
