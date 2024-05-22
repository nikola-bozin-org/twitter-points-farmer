CREATE TABLE IF NOT EXISTS tasks (
    id SERIAL PRIMARY KEY,
    description VARCHAR(255) NOT NULL,
    points INTEGER NOT NULL,
    time_created TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    link VARCHAR(120),
    task_button_text VARCHAR(120)
);
