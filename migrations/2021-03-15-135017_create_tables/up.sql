CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE customer_table (
    customer_table_id SERIAL PRIMARY KEY,
    code VARCHAR NOT NULL
);

CREATE TABLE menu (
    menu_id SERIAL PRIMARY KEY,
    item_name VARCHAR NOT NULL,
    cookTimeSeconds INT NOT NULL,
    price INT NOT NULL
);

CREATE TABLE session (
    session_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    customer_table_id INTEGER NOT NULL REFERENCES  customer_table (customer_table_id),
    start_timestamp TIMESTAMP NOT NULL DEFAULT current_timestamp,
    finish_timestamp TIMESTAMP DEFAULT NULL
);

CREATE TABLE customer_order (
    order_id SERIAL PRIMARY KEY,
    menu_id INTEGER NOT NULL REFERENCES menu (menu_id),
    session_id UUID NOT NULL REFERENCES session (session_id),
    customer_table_id INTEGER NOT NULL REFERENCES customer_table (customer_table_id),
    timestamp TIMESTAMP NOT NULL DEFAULT current_timestamp,
    completed BOOLEAN NOT NULL DEFAULT false
);

