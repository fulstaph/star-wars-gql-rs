-- Add migration script here
BEGIN;

CREATE TYPE filmmaker_profession AS ENUM ('Director', 'Scriptwriter', 'Producer', 'Cinematographer');

CREATE TABLE IF NOT EXISTS filmmakers (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    first_name VARCHAR(128) NOT NULL,
    last_name VARCHAR(128) NOT NULL,    
    profession filmmaker_profession NOT NULL
);

CREATE TABLE IF NOT EXISTS movies (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    title VARCHAR(128) NOT NULL,
    director_id BIGINT NOT NULL,
    scriptwriter_id BIGINT NOT NULL,
    producer_id BIGINT NOT NULL,
    release_date DATE NOT NULL,

    FOREIGN KEY (director_id) REFERENCES filmmakers (id),
    FOREIGN KEY (scriptwriter_id) REFERENCES filmmakers (id),
    FOREIGN KEY (producer_id) REFERENCES filmmakers (id)
);

CREATE TABLE IF NOT EXISTS starships (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    class VARCHAR(128) NOT NULL
);

CREATE TABLE IF NOT EXISTS planets (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    name VARCHAR(128) NOT NULL
);

CREATE TYPE race AS ENUM ('Human', 'Droid', 'Wookie');

CREATE TABLE IF NOT EXISTS characters (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    race race NOT NULL,
    starship_id BIGINT,

    FOREIGN KEY (starship_id) REFERENCES starships (id)
);

CREATE TABLE IF NOT EXISTS friends (
    character_id BIGINT NOT NULL,
    friend_character_id BIGINT NOT NULL,

    FOREIGN KEY (character_id) REFERENCES characters (id),
    FOREIGN KEY (friend_character_id) REFERENCES characters (id)
);

COMMIT;