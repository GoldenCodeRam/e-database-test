-- Your SQL goes here
CREATE TABLE GroundType (
    ground_type VARCHAR PRIMARY KEY
);

CREATE TABLE Parcel (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    address VARCHAR NOT NULL,
    size INTEGER NOT NULL,
    ground_type VARCHAR NOT NULL,
    available_water INTEGER NOT NULL
);

ALTER TABLE Parcel 
ADD CONSTRAINT available_water_check CHECK (available_water > 0);

ALTER TABLE Parcel
ADD CONSTRAINT ground_type_fk FOREIGN KEY (ground_type) REFERENCES GroundType (ground_type);
