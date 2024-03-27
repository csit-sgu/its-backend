CREATE TABLE author (
    author_id uuid NOT NULL PRIMARY KEY,
    name varchar(256) NOT NULL
);

CREATE TABLE book (
    book_id uuid NOT NULL PRIMARY KEY,
    title varchar(128) NOT NULL,
    description text DEFAULT NULL,
    author_id uuid NOT NULL,
    last_update timestamp with time zone NOT NULL DEFAULT now(),
    FOREIGN KEY (author_id) REFERENCES author(author_id)
        ON DELETE CASCADE
);
