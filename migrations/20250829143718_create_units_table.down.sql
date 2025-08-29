-- Add down migration script here
-- 1. Remove foreign key constraint
ALTER TABLE unit_types DROP CONSTRAINT IF EXISTS fk_base_unit;

-- 2. Drop units table first (because unit_types depends on it)
DROP TABLE IF EXISTS units;

-- 3. Drop unit_types table
DROP TABLE IF EXISTS unit_types;
