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
    created_at VARCHAR NOT NULL,
    last_updated_by VARCHAR NOT NULL,
    last_updated_at VARCHAR NOT NULL
);

CREATE TABLE authors (
    id SERIAL PRIMARY KEY,
    firstname VARCHAR NOT NULL,
    lastname VARCHAR NOT NULL,
    created_by VARCHAR NOT NULL,
    created_at VARCHAR NOT NULL,
    last_updated_by VARCHAR NOT NULL,
    last_updated_at VARCHAR NOT NULL
);

CREATE TABLE employees (
    id SERIAL PRIMARY KEY,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    firstname VARCHAR NOT NULL,
    lastname VARCHAR NOT NULL,
    role VARCHAR NOT NULL,
    login VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    created_by VARCHAR NOT NULL,
    created_at VARCHAR NOT NULL,
    last_updated_by VARCHAR NOT NULL,
    last_updated_at VARCHAR NOT NULL
);

CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    borrowed BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    name VARCHAR NOT NULL,
    created_by VARCHAR NOT NULL,
    created_at VARCHAR NOT NULL,
    last_updated_by VARCHAR NOT NULL,
    last_updated_at VARCHAR NOT NULL,
    FOREIGN KEY (author_id) REFERENCES authors(id),
    author_id INT NOT NULL,
    author_firstname VARCHAR NOT NULL,
    author_lastname VARCHAR NOT NULL
);

CREATE TABLE borrows (
    id SERIAL PRIMARY KEY,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    damaged BOOLEAN NOT NULL DEFAULT FALSE,
    borrow_date VARCHAR NOT NULL,
    limit_date VARCHAR NOT NULL,
    return_date VARCHAR NOT NULL DEFAULT '0000-00-00',
    created_by VARCHAR NOT NULL,
    created_at VARCHAR NOT NULL,
    last_updated_by VARCHAR NOT NULL,
    last_updated_at VARCHAR NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    user_id INT NOT NULL,
    FOREIGN KEY (book_id) REFERENCES books(id),
    book_id INT NOT NULL
);


INSERT INTO employees (
    is_active, 
    firstname, 
    lastname, 
    role, 
    login, 
    password, 
    created_by, 
    created_at, 
    last_updated_by, 
    last_updated_at
    ) 
    VALUES (
        TRUE, 
        'Loïc', 
        'Bouvier', 
        'employee', 
        'lbouvier', 
        '8c6976e5b5410415bde908bd4dee15dfb167a9c873fc4bb8a81f6f2ab448a918', 
        'admin', 
        '0000-00-00', 
        'admin', 
        '0000-00-00'
    );

INSERT INTO employees (
    is_active, 
    firstname, 
    lastname, 
    role, 
    login, 
    password, 
    created_by, 
    created_at, 
    last_updated_by, 
    last_updated_at
    ) 
    VALUES (
        TRUE, 
        'Martin', 
        'Reix', 
        'employee', 
        'mreix', 
        '8c6976e5b5410415bde908bd4dee15dfb167a9c873fc4bb8a81f6f2ab448a918', 
        'admin', 
        '0000-00-00', 
        'admin', 
        '0000-00-00'
    );