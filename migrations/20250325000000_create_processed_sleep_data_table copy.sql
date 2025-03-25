-- processed_sleep_data.sql
-- Table definition for storing processed sleep data and summaries

-- Create the table if it doesn't exist already
CREATE TABLE IF NOT EXISTS processed_sleep_data (
    id UUID PRIMARY KEY NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    data_type VARCHAR(50) NOT NULL,   -- 'sleep_stages', 'sleep_summary'
    night_date DATE NOT NULL,         -- Which night this sleep data belongs to
    data JSONB NOT NULL,              -- The processed sleep data
    created_at TIMESTAMPTZ NOT NULL,
    -- Composite index for efficient querying by user and date
    CONSTRAINT processed_sleep_data_user_id_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_processed_sleep_data_user_night_date ON processed_sleep_data(user_id, night_date);
CREATE INDEX IF NOT EXISTS idx_processed_sleep_data_data_type ON processed_sleep_data(data_type);
CREATE INDEX IF NOT EXISTS idx_processed_sleep_data_night_date ON processed_sleep_data(night_date);

-- Add database comments for documentation
COMMENT ON TABLE processed_sleep_data IS 'Stores sleep data processed by the Python analysis service';
COMMENT ON COLUMN processed_sleep_data.data_type IS 'Type of sleep data: sleep_stages, sleep_summary';
COMMENT ON COLUMN processed_sleep_data.night_date IS 'Date of the night for the sleep record (YYYY-MM-DD)';
COMMENT ON COLUMN processed_sleep_data.data IS 'JSON data containing sleep metrics, stages, and analysis';