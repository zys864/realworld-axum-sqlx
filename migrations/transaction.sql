CREATE OR REPLACE FUNCTION list_recent_article( user_id_ BIGINT, tag_ TEXT, 
author_ TEXT, favorited_ BOOLEAN, limit_ BIGINT, offset_ BIGINT) RETURNS 
TABLE( slug TEXT, title TEXT, description TEXT, body TEXT, taglist TEXT[], 
createdat TIMESTAMP WITH TIME ZONE, updatedat TIMESTAMP WITH TIME ZONE,
favorited BOOLEAN, favoritescount INT,username TEXT, bio TEXT, image TEXT, following BOOLEAN) 
AS $$
        SELECT
            slug,title,description,body,taglist,createdat,updatedat,
            (EXISTS(SELECT 1 FROM favorites WHERE favorites."user" = user_id_)) as favorited,
            (SELECT count(*) FROM favorites WHERE favorites.article=article.article_id) as favoritescount,
            username,bio,image,
            (SELECT article.author IN (SELECT followed FROM follows WHERE follower=user_id_)) as following
        FROM
            article
        INNER JOIN user_info ON article.author = user_info.user_id
        WHERE
            (tag_ IS NULL OR taglist @> ARRAY[tag_]) --tag filter
            AND(author_ IS NULL OR 
                    article.author = (SELECT user_info.user_id FROM user_info 
                        WHERE user_info.username = author_ )-- find arcticle user id by username then filter user author id
            ) --username filter
            AND(
                user_id_ is NULL OR favorited_ IS NULL OR
                (EXISTS(
                    SELECT 1
                    -- select the user(user_id_) all favorites articles as FROM data source
                    FROM (SELECT article FROM favorites WHERE favorites."user"=user_id_) AS f 
                    -- filter the article is the favorited
                    WHERE f.article = article.article_id
                ))
            ) -- favorited filter :require auth(user_id_)
            ORDER BY
                createdat DESC
            LIMIT
                NULLIF(limit_,20)
            OFFSET
                NULLIF(offset_,0)
$$ LANGUAGE SQL;