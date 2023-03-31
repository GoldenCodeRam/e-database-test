// @generated automatically by Diesel CLI.

diesel::table! {
    crop (id) {
        id -> Int4,
        parcel_id -> Int4,
        name -> Varchar,
        description -> Text,
        sowing_date -> Timestamp,
        estimated_harvest_date -> Timestamp,
        seed_amount -> Numeric,
        crop_type -> Varchar,
    }
}

diesel::table! {
    croptype (crop_type) {
        crop_type -> Varchar,
    }
}

diesel::table! {
    groundtype (ground_type) {
        ground_type -> Varchar,
    }
}

diesel::table! {
    harvest (id) {
        id -> Int4,
        crop_id -> Int4,
        date -> Timestamp,
        total_weight -> Numeric,
        price -> Numeric,
    }
}

diesel::table! {
    parcel (id) {
        id -> Int4,
        name -> Varchar,
        address -> Varchar,
        size -> Int4,
        ground_type -> Varchar,
        available_water -> Int4,
    }
}

diesel::table! {
    parcelowner (parcel_id, person_id) {
        parcel_id -> Int4,
        person_id -> Int4,
        acquisition_date -> Timestamp,
        succession -> Nullable<Timestamp>,
    }
}

diesel::table! {
    parcelworker (parcel_id, person_id) {
        parcel_id -> Int4,
        person_id -> Int4,
        hire_date -> Timestamp,
        position_type -> Varchar,
        salary -> Numeric,
        dismiss_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    paymenttype (payment_type) {
        payment_type -> Varchar,
    }
}

diesel::table! {
    person (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        address -> Varchar,
        phone -> Varchar,
        sex -> Int2,
    }
}

diesel::table! {
    positiontype (position_type) {
        position_type -> Varchar,
    }
}

diesel::table! {
    request (person_id, harvest_id) {
        person_id -> Int4,
        harvest_id -> Int4,
        date -> Timestamp,
        shipping_type -> Varchar,
        payment_type -> Varchar,
        request_status_type -> Varchar,
    }
}

diesel::table! {
    requeststatustype (request_status_type) {
        request_status_type -> Varchar,
    }
}

diesel::table! {
    shippingtype (shipping_type) {
        shipping_type -> Varchar,
    }
}

diesel::joinable!(crop -> parcel (parcel_id));
diesel::joinable!(harvest -> crop (crop_id));
diesel::joinable!(parcel -> groundtype (ground_type));
diesel::joinable!(parcelowner -> parcel (parcel_id));
diesel::joinable!(parcelowner -> person (person_id));
diesel::joinable!(parcelworker -> parcel (parcel_id));
diesel::joinable!(parcelworker -> person (person_id));
diesel::joinable!(request -> harvest (harvest_id));
diesel::joinable!(request -> person (person_id));

diesel::allow_tables_to_appear_in_same_query!(
    crop,
    croptype,
    groundtype,
    harvest,
    parcel,
    parcelowner,
    parcelworker,
    paymenttype,
    person,
    positiontype,
    request,
    requeststatustype,
    shippingtype,
);
