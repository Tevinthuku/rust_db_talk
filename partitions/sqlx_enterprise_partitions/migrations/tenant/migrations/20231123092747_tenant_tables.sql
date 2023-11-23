-- Add migration script here
CREATE TABLE hospital_visits (
    id SERIAL PRIMARY KEY,
    patient_id INTEGER NOT NULL REFERENCES public.fellow_kenyans (id),
    visit_date TIMESTAMP NOT NULL
)
