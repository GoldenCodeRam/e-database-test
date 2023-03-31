use diesel::{ExpressionMethods, PgConnection, RunQueryDsl};
use e_database_test::{
    establish_connection,
    models::{GroundType, NewParcel, NewPerson, Parcel, PositionType},
};
use fake::faker::{address::en::SecondaryAddress, company::en::Profession, lorem::en::Word};
use fake::faker::{company::en::BsAdj, phone_number::en::PhoneNumber};
use fake::faker::{company::en::BsNoun, name::en::*};
use fake::Fake;
use rand::{seq::SliceRandom, thread_rng, Rng};

const PERSON_COUNT: u8 = 20;
const PARCEL_COUNT: u8 = 10;
const TYPE_COUNT: u8 = 10;

const PERSON_SEX: [i16; 4] = [0, 1, 2, 9];

fn main() {
    let connection = &mut establish_connection();
    seed_person_table(connection);
    seed_parcel_table(connection);
    seed_parcel_person_table(connection);
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

    // seed ground types first
    (0..TYPE_COUNT).for_each(|_| {
        diesel::insert_into(groundtype::table)
            .values(ground_type.eq(&BsAdj().fake::<String>()))
            .execute(conn)
            .ok();
    });

    let ground_types = groundtype
        .load::<GroundType>(conn)
        .expect("Error loading ground types");
    (0..PARCEL_COUNT).for_each(|_| {
        let new_parcel = NewParcel {
            name: &Name().fake::<String>(),
            address: &SecondaryAddress().fake::<String>(),
            size: thread_rng().gen_range(0..100),
            ground_type: &ground_types.choose(&mut thread_rng()).unwrap().ground_type,
            available_water: thread_rng().gen_range(0..100),
        };

        diesel::insert_into(parcel::table)
            .values(&new_parcel)
            .get_result::<Parcel>(conn)
            .expect("Error saving parcel");
    });
}

fn seed_parcel_person_table(conn: &mut PgConnection) {
    use e_database_test::schema::positiontype;
    use e_database_test::schema::positiontype::dsl::*;
    (0..TYPE_COUNT).for_each(|_| {
        diesel::insert_into(positiontype::table)
            .values(position_type.eq(&Profession().fake::<String>()))
            .execute(conn)
            .ok();
    });

    let position_types = positiontype.load::<PositionType>(conn).expect("Error loading position types");
    (0..PERSON_COUNT).for_each(|i| {
        // use half users as workers and the other half as owners
        if i % 2 == 0 {
            let new_parcel_worker = NewParcelWorker {

            }
        }
    });
}
