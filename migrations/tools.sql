INSERT INTO article(slug,title,body,author)
VALUES('hello-hello','hello hello','body text',1007);
-- return user followed users by user id
CREATE OR REPLACE FUNCTION my_follwing_users(my_user_id BIGINT) RETURNS 
TABLE(
    followed BIGINT
)
AS
$$
    SELECT 
        followed
    FROM
        follows
    WHERE follows.follower = my_user_id
$$
LANGUAGE SQL;
-- return user followed users by user id
CREATE OR REPLACE FUNCTION get_user_base_info_by_user_id(id BIGINT) RETURNS
TABLE (
    username TEXT,
    bio TEXT,
    image TEXT
) AS
$$
    SELECT
        username,bio,image
    FROM 
        user_info
    WHERE
        user_info.user_id = id   
$$
LANGUAGE SQL;
