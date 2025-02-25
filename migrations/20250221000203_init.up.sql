-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS categories (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        name VARCHAR(255) NOT NULL UNIQUE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    IF NOT EXISTS products (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        category_id UUID,
        name VARCHAR(255) NOT NULL UNIQUE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    IF NOT EXISTS expenses (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        product_id UUID,
        price FLOAT NOT NULL DEFAULT 0.0,
        quantity FLOAT NOT NULL DEFAULT 1,
        measure VARCHAR(255) NOT NULL,
        expense_date DATE NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

ALTER TABLE products ADD CONSTRAINT products_category_id_fkey FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE expenses ADD CONSTRAINT expenses_product_id_fkey FOREIGN KEY (product_id) REFERENCES products (id) ON DELETE CASCADE ON UPDATE CASCADE;