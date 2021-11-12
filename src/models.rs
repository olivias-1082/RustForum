use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use super::schema::{users, posts, comments};
use dotenv::dotenv;
use argonautica::Hasher;
#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Post)]
#[belongs_to(User)]
pub struct Comment {
    pub id: i32,
    pub comment: String,
    pub post_id: i32,
    pub user_id: i32,
    pub parent_comment_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Insertable)]
#[table_name="comments"]
pub struct NewComment {
    pub comment: String,
    pub post_id: i32,
    pub user_id: i32,
    pub parent_comment_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}

impl NewComment {
    pub fn new(comment: String, post_id: i32,
        user_id: i32, parent_comment_id: Option<i32>) -> Self{
        NewComment {
            comment: comment,
            post_id: post_id,
            user_id: user_id,
            parent_comment_id: parent_comment_id,
            created_at: chrono::Local::now().naive_utc(),
        }
    }
}
#[derive(Serialize, Insertable)]
#[table_name="posts"]

pub struct NewPost {
    pub title: String,
    pub content: String,
    pub author: i32,
    pub created_at: chrono::NaiveDateTime,
}
impl NewPost {
    pub fn from_post_form(title: String, content: String, uid: i32) -> Self {
        NewPost {
            title: title,
            content: content,
            author: uid,
            created_at: chrono::Local::now().naive_utc(),
        }
    }
}
#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(User, foreign_key="author")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub author: i32,
    pub created_at: chrono::NaiveDateTime,
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}
impl NewUser {
    pub fn new(username: String, email: String, password: String) -> Self {
        dotenv().ok();

        let secret = std::env::var("SECRET_KEY")
            .expect("SECRET_KEY must be set");

        let hash = Hasher::default()
            .with_password(password)
            .with_secret_key(secret)
            .hash()
            .unwrap();

        NewUser {
            username: username,
            email: email,
            password: hash,
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}