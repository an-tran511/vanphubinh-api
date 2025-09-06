CREATE TABLE product_category (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    parent_product_category_id INT REFERENCES product_category(id) ON DELETE SET NULL
);