-- Migration to create the product_categories table
CREATE TABLE product_categories (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    parent_product_category_id UUID
);

ALTER TABLE product_categories
    ADD CONSTRAINT fk_parent_category
    FOREIGN KEY (parent_product_category_id)
    REFERENCES product_categories(id)
    ON DELETE SET NULL;


