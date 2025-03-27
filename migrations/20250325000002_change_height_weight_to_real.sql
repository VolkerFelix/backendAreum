-- Migration: Change height_cm and weight_kg columns to REAL type
ALTER TABLE user_profiles
    ALTER COLUMN height_cm TYPE REAL,
    ALTER COLUMN weight_kg TYPE REAL; 