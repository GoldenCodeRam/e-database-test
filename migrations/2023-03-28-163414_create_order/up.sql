-- Your SQL goes here
CREATE TABLE ShippingType (
    shipping_type VARCHAR PRIMARY KEY
);

CREATE TABLE PaymentType (
    payment_type VARCHAR PRIMARY KEY
);

CREATE TABLE RequestStatusType (
    request_status_type VARCHAR PRIMARY KEY
);

CREATE TABLE Request (
    person_id INTEGER NOT NULL,
    harvest_id INTEGER NOT NULL,
    date TIMESTAMP NOT NULL,
    shipping_type VARCHAR NOT NULL,
    payment_type VARCHAR NOT NULL,
    request_status_type VARCHAR NOT NULL,

    PRIMARY KEY (person_id, harvest_id)
);

ALTER TABLE Request ADD CONSTRAINT person_request_fk
FOREIGN KEY (person_id) REFERENCES Person (id);

ALTER TABLE Request ADD CONSTRAINT harvest_request_fk
FOREIGN KEY (harvest_id) REFERENCES Harvest (id);
