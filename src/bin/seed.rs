use diesel::{ExpressionMethods, PgConnection, RunQueryDsl};
use e_database_test::{
    establish_connection,
    models::{GroundType, NewParcel, NewPerson, Parcel},
};
use fake::faker::name::en::*;
use fake::faker::phone_number::en::PhoneNumber;
use fake::faker::{address::en::SecondaryAddress, lorem::en::Word};
use fake::Fake;
use rand::{seq::SliceRandom, thread_rng, Rng};

const PERSON_COUNT: u8 = 20;
const PARCEL_COUNT: u8 = 10;
const GROUND_TYPE_COUNT: u8 = 8;

const PERSON_SEX: [i16; 4] = [0, 1, 2, 9];

fn main() {
    let connection = &mut establish_connection();
    seed_person_table(connection);
    seed_parcel_table(connection);
}

fn seed_person_table(conn: &mut PgConnection) {
    use e_database_test::models::Person;
    use e_database_test::schema::person;

    (0..PERSON_COUNT).for_each(|_| {
        let new_person = NewPerson {
            first_name: &FirstName().fake::<String>(),
            last_name: &LastName().fake::<String>(),
            address: &SecondaryAddress().fake::<String>(),
            phone: &PhoneNumber().fake::<String>(),
            sex: *PERSON_SEX.choose(&mut thread_rng()).unwrap(),
        };

        diesel::insert_into(person::table)
            .values(&new_person)
            .get_result::<Person>(conn)
            .expect("Error saving person");
    });
}

fn seed_parcel_table(conn: &mut PgConnection) {
    use e_database_test::schema::groundtype;
    use e_database_test::schema::groundtype::dsl::*;
    use e_database_test::schema::parcel;

    (0..GROUND_TYPE_COUNT).for_each(|_| {
        diesel::insert_into(groundtype::table)
            .values(ground_type.eq(&Word().fake::<String>()))
            .get_result::<GroundType>(conn)
            .expect("Error saving ground type");
    });

    let ground_types = groundtype
        .load::<GroundType>(conn)
        .expect("Error loading ground types");
    (0..PARCEL_COUNT).for_each(|_| {
        println!(
            "{}",
            ground_types.choose(&mut thread_rng()).unwrap().ground_type
        );
        let new_parcel = NewParcel {
            name: &Name().fake::<String>(),
            address: &SecondaryAddress().fake::<String>(),
            size: thread_rng().gen::<i32>(),
            ground_type: &ground_types.choose(&mut thread_rng()).unwrap().ground_type,
            available_water: thread_rng().gen::<i32>(),
        };

        diesel::insert_into(parcel::table)
            .values(&new_parcel)
            .get_result::<Parcel>(conn)
            .expect("Error saving parcel");
    });
}
