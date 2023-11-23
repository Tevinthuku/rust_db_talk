-- Add migration script here
ALTER TABLE hospital_visits
ALTER COLUMN visit_date
SET DEFAULT NOW();
