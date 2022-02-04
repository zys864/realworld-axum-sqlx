-- Add up migration script here

CREATE TABLE IF NOT EXISTS user_info
(
    user_id     BIGSERIAL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL,
    bio TEXT ,
    image TEXT,
    hashed_password text NOT NULL
);
CREATE TABLE IF NOT EXISTS article(
    article_id BIGSERIAL PRIMARY KEY,
    slug text NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    body TEXT NOT NULL,
    taglist TEXT[],
    createdAt TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updatedAt TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    favoritesCount INTEGER NOT NULL DEFAULT 0,
    author BIGINT NOT NULL REFERENCES user_info(user_id) ON DELETE CASCADE 
);
CREATE TABLE IF NOT EXISTS follows (
       follower BIGINT REFERENCES user_info(user_id) ON DELETE CASCADE,
       followed BIGINT REFERENCES user_info(user_id) ON DELETE CASCADE,
       CHECK (follower != followed),
       PRIMARY KEY(follower, followed)
);
CREATE TABLE IF NOT EXISTS comments (
       comment_id SERIAL PRIMARY KEY,
       body TEXT NOT NULL,
       article BIGINT NOT NULL REFERENCES article ON DELETE CASCADE,
       author BIGINT NOT NULL REFERENCES user_info ON DELETE CASCADE,
       created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE TABLE IF NOT EXISTS favorites (
       "user" INTEGER REFERENCES user_info(user_id) ON DELETE CASCADE,
       article INTEGER REFERENCES article(article_id) ON DELETE CASCADE,
       PRIMARY KEY ("user", article)
);