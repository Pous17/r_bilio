CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    member BOOLEAN NOT NULL DEFAULT FALSE,
    score INT NOT NULL DEFAULT 3
);

CREATE TABLE author (
    id SERIAL PRIMARY KEY,
    firstname VARCHAR NOT NULL,
    lastname VARCHAR NOT NULL
);

CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    borrowed BOOLEAN NOT NULL DEFAULT FALSE,
    author_id INT NOT NULL,
    author_firstname VARCHAR NOT NULL,
    author_lastname VARCHAR NOT NULL,
    FOREIGN KEY (author_id) REFERENCES author(id)
);

CREATE TABLE employees (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE borrows (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    book_id INT NOT NULL,
    borrow_date VARCHAR NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (book_id) REFERENCES books(id)
);

CREATE TABLE past_borrows (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    book_id INT NOT NULL,
    condition BOOLEAN NOT NULL,
    borrow_date VARCHAR NOT NULL,
    return_date VARCHAR NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (book_id) REFERENCES books(id)   
)