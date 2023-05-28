-- Your SQL goes here
CREATE TABLE users(
  username TEXT NOT NULL PRIMARY KEY UNIQUE,
  password TEXT NOT NULL
);
INSERT INTO users VALUES("admin", "admin");
