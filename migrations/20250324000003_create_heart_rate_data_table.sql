-- Create heart rate data table
CREATE TABLE IF NOT EXISTS heart_rate_data (
    id UUID PRIMARY KEY NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    device_info JSONB NOT NULL,
    sampling_rate_hz INTEGER NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    heart_rate_values INTEGER[] NOT NULL,
    timestamps TIMESTAMPTZ[] NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT heart_rate_data_user_id_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT heart_rate_data_time_range CHECK (end_time >= start_time),
    CONSTRAINT heart_rate_data_array_lengths CHECK (array_length(heart_rate_values, 1) = array_length(timestamps, 1))
);

-- Create index for faster queries by user_id and time range
CREATE INDEX IF NOT EXISTS idx_heart_rate_data_user_time ON heart_rate_data(user_id, start_time, end_time); 