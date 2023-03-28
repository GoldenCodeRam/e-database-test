use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub phone: String,
    // ISO/IEC 5218
    pub sex: u8,
}
