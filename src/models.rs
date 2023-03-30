use diesel::prelude::*;

use crate::schema::{person, parcel};

#[derive(Queryable)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub phone: String,
    // ISO/IEC 5218
    pub sex: i16,
}

#[derive(Insertable)]
#[diesel(table_name=person)]
pub struct NewPerson<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub address: &'a str,
    pub phone: &'a str,
    // ISO/IEC 5218
    pub sex: i16,
}

#[derive(Queryable)]
pub struct Parcel {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub size: i32,
    pub ground_type: String,
    pub available_water: i32,
} 

#[derive(Insertable)]
#[diesel(table_name=parcel)]
pub struct NewParcel<'a> {
    pub name: &'a str,
    pub address: &'a str,
    pub size: i32,
    pub ground_type: &'a str,
    pub available_water: i32,
}

#[derive(Queryable)]
pub struct GroundType {
    pub ground_type: String,
}
