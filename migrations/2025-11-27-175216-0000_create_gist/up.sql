-- Your SQL goes here
CREATE INDEX people_search_index
    ON people
    USING GIST (search gist_trgm_ops);
