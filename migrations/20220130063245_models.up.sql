-- Add up migration script here

CREATE TABLE IF NOT EXISTS user_info
(
    id     BIGSERIAL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL,
    bio TEXT ,
    image TEXT,
    hashed_password text NOT NULL
);
CREATE TABLE IF NOT EXISTS article(
    id BIGSERIAL PRIMARY KEY,
    slug text NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    body TEXT NOT NULL,
    taglist TEXT[],
    createdAt TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updatedAt TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    favoritesCount INTEGER NOT NULL DEFAULT 0,
    author BIGINT NOT NULL REFERENCES user_info ON DELETE CASCADE 
);
CREATE TABLE follows (
       follower BIGINT REFERENCES user_info ON DELETE CASCADE,
       followed BIGINT REFERENCES user_info ON DELETE CASCADE,
       CHECK (follower != followed),
       PRIMARY KEY(follower, followed)
);
CREATE TABLE comments (
       id SERIAL PRIMARY KEY,
       body TEXT NOT NULL,
       article BIGINT NOT NULL REFERENCES article ON DELETE CASCADE,
       author BIGINT NOT NULL REFERENCES user_info ON DELETE CASCADE,
       created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);