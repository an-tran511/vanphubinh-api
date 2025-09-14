-- Add up migration script here
CREATE TABLE units (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);
