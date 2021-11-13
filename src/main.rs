use sqlx::postgres::PgPool;
use std::env;
use structopt::StructOpt;
#[macro_use]
use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use std::io::{self, Read};

#[derive(Debug, Deserialize)]
struct Row {
    id: i64,
    User: Json<User>,
}
struct User {
    username: String,
    email: String,
    password: String,
}
#[derive(Debug, Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

async fn signup(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Sign Up");

    let rendered = tera.render("signup.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
    
}


#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    content: String,
}
#[derive(Deserialize)]
struct PostForm {
    title: String,
    content: String,
}

async fn submission(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Submit a Post");

    let rendered = tera.render("submission.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn process_submission(post:web::Form<Submission>) ->  anyhow::Result<i64>{
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let rec = sqlx::query!(
        r#"
INSERT INTO posts ( post )
VALUES ( $1 )
RETURNING id
        "#,
        post
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id);
    
    HttpResponse::Ok().body(format!("Posted submission: {}", post.title))

}
async fn login(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Login");

    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}
async fn process_login(data: web::Form<LoginUser>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Logged in: {}", data.username))
}


async fn process_signup(user: web::Form<User>) -> impl Responder {
    println!("{:?}", user);
    let rec = sqlx::query!(
        r#"
INSERT INTO users ( data )
VALUES ( $1 )
RETURNING id
        "#,
        user
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", user.username))
}

#[derive(Serialize)]
struct Post {
    title: String,
    content: String,
    author: String,
}
async fn index(tera: web::Data<Tera>) -> impl Responder {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let mut data = Context::new();

    let posts = [
        Post {
            title: String::from("This is the title"),
            content: String::from("This is the content"),
            author: String::from("Bob")
        }
    ];
    

    data.insert("title", "Hacker Clone");
    data.insert("posts", &posts);

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}
async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().body("Logged out.")
}
#[async_std::main]
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(tera)
            .route("/", web::get().to(index))
            .route("/signup", web::get().to(signup))
            .route("/signup", web::post().to(process_signup))    })
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(process_login))
            .route("/submission", web::get().to(submission))
            .route("/submission", web::post().to(process_submission))
    .bind("localhost:2000")?
    .run()
    .await
}
   