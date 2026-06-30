-- Add migration script here
CREATE TYPE e_user_visibility AS ENUM ('PUBLIC', 'PROTECTED', 'PRIVATE');

CREATE TABLE users
(
    id         UUID                     DEFAULT gen_random_uuid() NOT NULL,
    name       VARCHAR(256)                                       NOT NULL,
    username   VARCHAR(64),
    rating     NUMERIC (5, 2),
    visibility e_user_visibility        DEFAULT 'PUBLIC'          NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()             NOT NULL
);

ALTER TABLE users
    ADD CONSTRAINT pk_user PRIMARY KEY (id);

CREATE UNIQUE INDEX idx_user_username ON users (username);
