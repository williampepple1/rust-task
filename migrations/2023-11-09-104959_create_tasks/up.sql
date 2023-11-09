-- Your SQL goes here
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description TEXT,
    completed BOOLEAN NOT NULL DEFAULT 'f'
);


