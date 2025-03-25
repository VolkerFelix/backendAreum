-- Migration: Create user_profiles table
-- This table extends the existing users table with additional profile information
CREATE TABLE IF NOT EXISTS user_profiles (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    display_name VARCHAR(100), -- Name or nickname
    date_of_birth DATE,
    biological_sex VARCHAR(20), -- 'male', 'female', 'other', 'prefer_not_to_say'
    height_cm DECIMAL(5,2),
    weight_kg DECIMAL(5,2),
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT user_profiles_user_id_unique UNIQUE (user_id)
);

-- Create user_goals table (many-to-many relationship)
CREATE TABLE IF NOT EXISTS goal_types (
    id UUID PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL
);

-- Create user_goals table (many-to-many relationship)
CREATE TABLE IF NOT EXISTS user_goals (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    goal_type_id UUID NOT NULL REFERENCES goal_types(id),
    priority INTEGER NOT NULL DEFAULT 1, -- 1 is highest priority
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    UNIQUE (user_id, goal_type_id)
);

-- Create lifestyle_info table
CREATE TABLE IF NOT EXISTS lifestyle_info (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    activity_level VARCHAR(20) NOT NULL, -- 'sedentary', 'lightly_active', 'active', 'very_active'
    bedtime TIME,
    wake_time TIME,
    is_smoker BOOLEAN,
    alcohol_consumption VARCHAR(20), -- 'none', 'occasional', 'moderate', 'frequent'
    tracks_menstrual_cycle BOOLEAN,
    menstrual_cycle_data JSONB, -- Optional menstrual cycle tracking data
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT lifestyle_info_user_id_unique UNIQUE (user_id)
);

-- Create medical_conditions table
CREATE TABLE IF NOT EXISTS medical_condition_types (
    id UUID PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL
);

-- Create user_medical_conditions table (many-to-many relationship)
CREATE TABLE IF NOT EXISTS user_medical_conditions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    condition_id UUID NOT NULL REFERENCES medical_condition_types(id),
    diagnosed_at DATE,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    UNIQUE (user_id, condition_id)
);

-- Create permissions_settings table
CREATE TABLE IF NOT EXISTS permissions_settings (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    heart_rate_enabled BOOLEAN NOT NULL DEFAULT false,
    temperature_enabled BOOLEAN NOT NULL DEFAULT false,
    spo2_enabled BOOLEAN NOT NULL DEFAULT false,
    accelerometer_enabled BOOLEAN NOT NULL DEFAULT false,
    notifications_enabled BOOLEAN NOT NULL DEFAULT false,
    background_usage_enabled BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT permissions_settings_user_id_unique UNIQUE (user_id)
);

-- Create third_party_connections table
CREATE TABLE IF NOT EXISTS third_party_connection_types (
    id UUID PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL
);

-- Create user_third_party_connections table (many-to-many relationship)
CREATE TABLE IF NOT EXISTS user_third_party_connections (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    connection_type_id UUID NOT NULL REFERENCES third_party_connection_types(id),
    access_token TEXT,
    refresh_token TEXT,
    token_expires_at TIMESTAMPTZ,
    connection_status VARCHAR(20) NOT NULL, -- 'connected', 'pending', 'disconnected', 'failed'
    last_sync_at TIMESTAMPTZ,
    connection_data JSONB, -- Optional additional connection data
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    UNIQUE (user_id, connection_type_id)
);

-- Create personalization_info table
CREATE TABLE IF NOT EXISTS personalization_info (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    stress_triggers TEXT[],  -- Array of stress triggers
    work_type VARCHAR(20),  -- 'office', 'remote', 'shift_based', 'student'
    daily_routine JSONB,  -- JSON structure for daily routine tags
    timezone VARCHAR(50),  -- User's timezone for circadian rhythm analysis
    location_data JSONB,  -- Optional location data
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT personalization_info_user_id_unique UNIQUE (user_id)
);

-- Create onboarding_progress table to track user's progress through onboarding steps
CREATE TABLE IF NOT EXISTS onboarding_progress (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    basic_info_completed BOOLEAN NOT NULL DEFAULT false,
    lifestyle_health_completed BOOLEAN NOT NULL DEFAULT false,
    permissions_setup_completed BOOLEAN NOT NULL DEFAULT false,
    personalization_completed BOOLEAN NOT NULL DEFAULT false,
    onboarding_completed BOOLEAN NOT NULL DEFAULT false,
    current_step VARCHAR(50) NOT NULL DEFAULT 'basic_info', -- Current onboarding step
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT onboarding_progress_user_id_unique UNIQUE (user_id)
);

-- Insert default goal types
INSERT INTO goal_types (id, name, description, created_at)
VALUES
    (gen_random_uuid(), 'stress_reduction', 'Reduce stress and improve mental wellbeing', NOW()),
    (gen_random_uuid(), 'sleep_improvement', 'Improve sleep quality and duration', NOW()),
    (gen_random_uuid(), 'fitness_activity', 'Increase physical fitness and activity levels', NOW()),
    (gen_random_uuid(), 'general_wellness', 'Improve overall health and wellness', NOW())
ON CONFLICT (name) DO NOTHING;

-- Insert default medical condition types
INSERT INTO medical_condition_types (id, name, description, created_at)
VALUES
    (gen_random_uuid(), 'insomnia', 'Difficulty falling asleep or staying asleep', NOW()),
    (gen_random_uuid(), 'hypertension', 'High blood pressure', NOW()),
    (gen_random_uuid(), 'anxiety', 'Anxiety disorder', NOW()),
    (gen_random_uuid(), 'depression', 'Depression', NOW()),
    (gen_random_uuid(), 'diabetes', 'Diabetes (type 1 or 2)', NOW()),
    (gen_random_uuid(), 'asthma', 'Asthma', NOW()),
    (gen_random_uuid(), 'migraine', 'Migraine headaches', NOW()),
    (gen_random_uuid(), 'sleep_apnea', 'Sleep apnea', NOW())
ON CONFLICT (name) DO NOTHING;

-- Insert default third-party connection types
INSERT INTO third_party_connection_types (id, name, description, created_at)
VALUES
    (gen_random_uuid(), 'apple_health', 'Apple Health integration', NOW()),
    (gen_random_uuid(), 'google_fit', 'Google Fit integration', NOW()),
    (gen_random_uuid(), 'fitbit', 'Fitbit integration', NOW()),
    (gen_random_uuid(), 'garmin', 'Garmin integration', NOW())
ON CONFLICT (name) DO NOTHING;

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_user_profiles_user_id ON user_profiles(user_id);
CREATE INDEX IF NOT EXISTS idx_user_goals_user_id ON user_goals(user_id);
CREATE INDEX IF NOT EXISTS idx_lifestyle_info_user_id ON lifestyle_info(user_id);
CREATE INDEX IF NOT EXISTS idx_user_medical_conditions_user_id ON user_medical_conditions(user_id);
CREATE INDEX IF NOT EXISTS idx_permissions_settings_user_id ON permissions_settings(user_id);
CREATE INDEX IF NOT EXISTS idx_user_third_party_connections_user_id ON user_third_party_connections(user_id);
CREATE INDEX IF NOT EXISTS idx_personalization_info_user_id ON personalization_info(user_id);
CREATE INDEX IF NOT EXISTS idx_onboarding_progress_user_id ON onboarding_progress(user_id);