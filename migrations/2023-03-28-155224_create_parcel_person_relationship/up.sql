-- Your SQL goes here
CREATE TABLE ParcelOwner (
    parcel_id INTEGER NOT NULL,
    person_id INTEGER NOT NULL,

    acquisition_date TIMESTAMP NOT NULL,

    succession TIMESTAMP,
    PRIMARY KEY (parcel_id, person_id)
);

ALTER TABLE ParcelOwner ADD CONSTRAINT parcel_parcel_owner_fk
FOREIGN KEY (parcel_id) REFERENCES Parcel (id);

ALTER TABLE ParcelOwner ADD CONSTRAINT person_parcel_owner_fk
FOREIGN KEY (person_id) REFERENCES Person (id);

-- Parcel Worker table

CREATE TABLE PositionType (
    position_type VARCHAR PRIMARY KEY
);

CREATE TABLE ParcelWorker (
    parcel_id INTEGER NOT NULL,
    person_id INTEGER NOT NULL,

    hire_date TIMESTAMP NOT NULL,
    position_type VARCHAR NOT NULL,
    salary NUMERIC NOT NULL,

    dismiss_date TIMESTAMP,
    PRIMARY KEY (parcel_id, person_id)
);

ALTER TABLE ParcelWorker ADD CONSTRAINT salary_check
CHECK (salary > 0);

ALTER TABLE ParcelWorker ADD CONSTRAINT parcel_parcel_worker_fk
FOREIGN KEY (parcel_id) REFERENCES Parcel (id);

ALTER TABLE ParcelWorker ADD CONSTRAINT person_parcel_worker_fk
FOREIGN KEY (person_id) REFERENCES Person (id);
