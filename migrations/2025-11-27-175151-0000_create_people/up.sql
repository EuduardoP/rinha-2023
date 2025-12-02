-- Your SQL goes here
CREATE TABLE people (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    nick VARCHAR(32) NOT NULL,
    birthday DATE NOT NULL,
    stack TEXT[] NULL,
    search TEXT GENERATED ALWAYS AS (
        name || ' ' || nick || ' ' || COALESCE(ARRAY_TO_STRING_IMMUTABLE(stack, ' '), '')
    ) STORED,
    CONSTRAINT unique_nick UNIQUE (nick)
);
