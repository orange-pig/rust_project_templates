-- Active: 666666666666@@127.0.0.1@5432
CREATE DATABASE your_db_name

CREATE TABLE users (
    id serial PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL
);

INSERT INTO
    "users" ("id", "name", "email")
VALUES (1, 'user1', 'user1@mail.com,'),
    (2, 'user2', 'user2@mail.com,'),
    (3, 'user3', 'user3@mail.com,');
