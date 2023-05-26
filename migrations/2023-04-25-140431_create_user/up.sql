CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    score INT NOT NULL DEFAULT 3,
    member BOOLEAN NOT NULL DEFAULT TRUE,
    firstname VARCHAR NOT NULL,
    lastname VARCHAR NOT NULL,
    role VARCHAR NOT NULL,
    login VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    created_by VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    last_updated_by VARCHAR NOT NULL,
    last_updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
);

CREATE TABLE authors (
    id SERIAL PRIMARY KEY,
    firstname VARCHAR NOT NULL,
    lastname VARCHAR NOT NULL,
    created_by VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    last_updated_by VARCHAR NOT NULL,
    last_updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
);

CREATE TABLE employees (
    id SERIAL PRIMARY KEY,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    firstname VARCHAR NOT NULL,
    lastname VARCHAR NOT NULL,
    role VARCHAR NOT NULL,
    login VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    created_by VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    last_updated_by VARCHAR NOT NULL,
    last_updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
);

CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    borrowed BOOLEAN NOT NULL DEFAULT FALSE,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    name VARCHAR NOT NULL,
    created_by VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    last_updated_by VARCHAR NOT NULL,
    last_updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    FOREIGN KEY (author_id) REFERENCES author(id),
    author_id INT NOT NULL,
    author_firstname VARCHAR NOT NULL,
    author_lastname VARCHAR NOT NULL,
);



CREATE TABLE borrows (
    id SERIAL PRIMARY KEY,
    borrow_date TIMESTAMP NOT NULL DEFAULT current_timestamp,
    limit_date TIMESTAMP NOT NULL,
    created_by VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    last_updated_by VARCHAR NOT NULL,
    last_updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    FOREIGN KEY (user_id) REFERENCES users(id),
    user_id INT NOT NULL,
    FOREIGN KEY (book_id) REFERENCES books(id),
    book_id INT NOT NULL
);

CREATE TABLE past_borrows (
    id SERIAL PRIMARY KEY,
    condition BOOLEAN NOT NULL,
    borrow_date VARCHAR NOT NULL,
    limit_date VARCHAR NOT NULL,
    return_date TIMESTAMP NOT NULL DEFAULT current_timestamp,
    created_by VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    last_updated_by VARCHAR NOT NULL,
    last_updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    FOREIGN KEY (user_id) REFERENCES users(id),
    user_id INT NOT NULL,
    FOREIGN KEY (book_id) REFERENCES books(id),
    book_id INT NOT NULL, 
)