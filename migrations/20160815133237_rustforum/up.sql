CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,

    UNIQUE(username),
    UNIQUE(email)
);


CREATE TABLE replies (
    id SERIAL PRIMARY KEY,
    reply VARCHAR NOT NULL,
    post_id INT NOT NULL,
    user_id INT NOT NULL,
    parent_comment_id INT,
    creation_time TIMESTAMP NOT NULL,

    CONSTRAINT fk_post
        FOREIGN KEY(post_id)
            REFERENCES posts(id),

    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_parent_comment
        FOREIGN KEY(parent_comment_id)
            REFERENCES replies(id)
);
CREATE TABLE replies (
    id SERIAL PRIMARY KEY,
    reply VARCHAR NOT NULL,
    post_id INT NOT NULL,
    user_id INT NOT NULL,
    parent_comment_id INT,
    creation_time TIMESTAMP NOT NULL,

    CONSTRAINT fk_post
        FOREIGN KEY(post_id)
            REFERENCES posts(id),

    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_parent_comment
        FOREIGN KEY(parent_comment_id)
            REFERENCES replies(id)
);
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    content VARCHAR,
    author INT NOT NULL,
    creation_time TIMESTAMP NOT NULL,

    CONSTRAINT fk_author
        FOREIGN KEY(author)
            REFERENCES users(id)
);