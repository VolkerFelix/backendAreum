-- Create acceleration data table
CREATE TABLE IF NOT EXISTS acceleration_data (
    id UUID PRIMARY KEY NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    device_info JSONB NOT NULL,
    sampling_rate_hz INTEGER NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    acceleration_values JSONB NOT NULL,
    timestamps TIMESTAMPTZ[] NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT acceleration_data_user_id_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT acceleration_data_time_range CHECK (end_time >= start_time),
    CONSTRAINT acceleration_data_array_lengths CHECK (array_length(timestamps, 1) = (acceleration_values->>'x')::jsonb_array_length())
);

-- Create index for faster queries by user_id and time range
CREATE INDEX IF NOT EXISTS idx_acceleration_data_user_time ON acceleration_data(user_id, start_time, end_time); 