-- Add migration script here
CREATE TABLE fbsearch(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    searched_at timestamptz NOT NULL
);