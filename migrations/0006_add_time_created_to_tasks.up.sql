ALTER TABLE tasks
ADD COLUMN time_created TIMESTAMP WITH TIME ZONE DEFAULT NOW();
