-- Add up migration script here
-- Categories of units (UoM Categories in Odoo)
CREATE TABLE unit_types (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,         -- e.g., Weight, Length, Volume
    base_unit_id UUID UNIQUE            -- reference unit for this category
);

-- Units of measure
CREATE TABLE units (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,         -- e.g., Kilogram, Gram, Ton
    symbol TEXT NOT NULL,              -- e.g., kg, g, t
    unit_type_id UUID NULL REFERENCES unit_types(id) ON DELETE CASCADE,
    factor NUMERIC(20,10) NOT NULL,    -- ratio to base unit (base unit = 1.0)
    is_active BOOLEAN DEFAULT TRUE
);

ALTER TABLE unit_types
ADD CONSTRAINT fk_base_unit FOREIGN KEY (base_unit_id) REFERENCES units(id) ON DELETE CASCADE;
