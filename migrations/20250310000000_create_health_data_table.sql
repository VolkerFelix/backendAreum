-- Add migration script here
CREATE TABLE IF NOT EXISTS health_data (
    id UUID PRIMARY KEY NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    data_type VARCHAR(50) NOT NULL,
    device_info JSONB NOT NULL,
    sampling_rate_hz INTEGER NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    data JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT health_data_user_id_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);