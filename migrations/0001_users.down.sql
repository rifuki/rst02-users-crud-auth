-- Add down migration script here
DROP TABLE users;

DROP INDEX idx_username ON users;