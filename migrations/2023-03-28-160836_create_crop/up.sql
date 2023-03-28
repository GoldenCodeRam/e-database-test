-- Your SQL goes here
CREATE TABLE CropType (
    crop_type VARCHAR PRIMARY KEY
);

CREATE TABLE Crop (
    id SERIAL PRIMARY KEY,
    parcel_id INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    description TEXT NOT NULL,
    sowing_date TIMESTAMP NOT NULL,
    estimated_harvest_date TIMESTAMP NOT NULL,
    seed_amount NUMERIC NOT NULL,
    crop_type VARCHAR NOT NULL
);

ALTER TABLE Crop ADD CONSTRAINT seed_amount_check
CHECK (seed_amount > 0);

ALTER TABLE Crop ADD CONSTRAINT parcel_crop_fk
FOREIGN KEY (parcel_id) REFERENCES Parcel (id);

-- Harvest table

CREATE TABLE Harvest (
    id SERIAL PRIMARY KEY,
    crop_id INTEGER NOT NULL,
    date TIMESTAMP NOT NULL,
    total_weight NUMERIC NOT NULL
);

ALTER TABLE Harvest ADD CONSTRAINT total_weight_check
CHECK (total_weight > 0);

ALTER TABLE Harvest ADD CONSTRAINT crop_harvest_fk
FOREIGN KEY (crop_id) REFERENCES Crop (id);

