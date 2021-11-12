#[macro_use]
extern crate diesel;
pub mod schema;
pub mod models;
use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use models::{User, NewUser, LoginUser, Post, NewPost, Comment, NewComment};
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};

#[derive(Deserialize)]
struct CommentForm {
    comment: String,
}
#[derive(Debug)]
enum ServerError {
    ArgonauticError,
    DieselError,
    EnvironmentError,
    R2D2Error,
    UserError(String)
}



impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f, "Test")
    }
}
impl From<r2d2::Error> for ServerError {
    fn from(_: r2d2::Error) -> ServerError {
        ServerError::R2D2Error
    }
}
impl From<std::env::VarError> for ServerError {
    fn from(_: std::env::VarError) -> ServerError {
        ServerError::EnvironmentError
    }
}
impl actix_web::error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::ArgonauticError => HttpResponse::InternalServerError().json("Argonautica Error."),
            ServerError::DieselError => HttpResponse::InternalServerError().json("Diesel Error."),
            ServerError::EnvironmentError => HttpResponse::InternalServerError().json("Environment Error."),
            ServerError::UserError(data) => HttpResponse::InternalServerError().json(data),
            ServerError::R2D2Error => todo!(),
        }
    }
}
impl From<diesel::result::Error> for ServerError {
    fn from(err: diesel::result::Error) -> ServerError {
        match err {
            diesel::result::Error::NotFound => ServerError::UserError("Username not found.".to_string()),
            _ => ServerError::DieselError
        }
    }
}
async fn comment(
    data: web::Form<CommentForm>,
    id: Identity,
    web::Path(post_id): web::Path<i32>
) -> impl Responder {

    if let Some(id) = id.identity() {
        use schema::posts::dsl::{posts};
        use schema::users::dsl::{users, username};

        let connection = pool.get()?;

        let post :Post = posts.find(post_id)
            .get_result(&connection)
            .expect("Failed to find post.");

            let user = users.filter(username.eq(&data.username)).first::<User>(&connection);
        
        match user {
            Ok(u) => {
                let parent_id = None;
                let new_comment = NewComment::new(data.comment.clone(), post.id, u.id, parent_id);

                use schema::comments;
                diesel::insert_into(comments::table)
                    .values(&new_comment)
                    .get_result::<Comment>(&connection)
                    .expect("Error saving comment.");


                return HttpResponse::Ok().body("Commented.");
            }
            Err(e) => {
                println!("{:?}", e);
                return HttpResponse::Ok().body("User not found.");
            }
        }
    }

    HttpResponse::Unauthorized().body("Not logged in.")
}
fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

async fn post_page(tera: web::Data<Tera>,
    id: Identity,
    web::Path(post_id): web::Path<i32>
) -> impl Responder {

    use schema::posts::dsl::{posts};
    use schema::users::dsl::{users};

    let connection = establish_connection();

    let post :Post = posts.find(post_id)
        .get_result(&connection)
        .expect("Failed to find post.");

    let user :User = users.find(post.author)
        .get_result(&connection)
        .expect("Failed to find user.");

        let comments :Vec<(Comment, User)> = Comment::belonging_to(&post)
        .inner_join(users)
        .load(&connection)
        .expect("Failed to find comments.");

    let mut data = Context::new();
    data.insert("title", &format!("{} - HackerClone", post.title));
    data.insert("post", &post);
    data.insert("user", &user);
    data.insert("comments", &comments);

    if let Some(_id) = id.identity() {
        data.insert("logged_in", "true");
    } else {
        data.insert("logged_in", "false");
    }

    let rendered = tera.render("post.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}
#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    link: String,
}
#[derive(Deserialize)]
struct PostForm {
    title: String,
    link: String,
}

async fn submission(tera: web::Data<Tera>, id: Identity) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Submit a Post");

    if let Some(id) = id.identity() {
        let rendered = tera.render("submission.html", &data).unwrap();
        return HttpResponse::Ok().body(rendered);
    }

    HttpResponse::Unauthorized().body("User not logged in.")
}

async fn process_submission(data: web::Form<PostForm>, id: Identity) -> impl Responder {
    if let Some(id) = id.identity() {
        use schema::users::dsl::{username, users};

        let connection = establish_connection();
        let user :Result<User, diesel::result::Error> = users.filter(username.eq(id)).first(&connection);

        match user {
            Ok(u) => {
                let new_post = NewPost::from_post_form(data.title.clone(), data.link.clone(), u.id);

                use schema::posts;

                diesel::insert_into(posts::table)
                    .values(&new_post)
                    .get_result::<Post>(&connection)
                    .expect("Error saving post.");

                return HttpResponse::Ok().body("Submitted.");
            }
            Err(e) => {
                println!("{:?}", e);
                return HttpResponse::Ok().body("Failed to find user.");
            }
        }
    }
    HttpResponse::Unauthorized().body("User not logged in.")
}
async fn login(tera: web::Data<Tera>, id: Identity) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Login");

  
    if u.password == data.password {
        let session_token = String::from(u.username);
        id.remember(session_token);
        HttpResponse::Ok().body(format!("Logged in: {}", data.username))
    } else {
        HttpResponse::Ok().body("Password is incorrect.")
    }
    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}
async fn process_login(data: web::Form<LoginUser>, id: Identity, pool: web::Data<Pool>) -> Result<HttpResponse, ServerError> {
    use schema::users::dsl::{username, users};

    let connection = pool.get()?;
    let user = users.filter(username.eq(&data.username)).first::<User>(&connection)?;

    dotenv().ok();
    let secret = std::env::var("SECRET_KEY")?;

    let valid = Verifier::default()
    .with_hash(user.password)
    .with_password(data.password.clone())
    .with_secret_key(secret)
    .verify()?;

    if valid {
        let session_token = String::from(user.username);
        id.remember(session_token);
        Ok(HttpResponse::Ok().body(format!("Logged in: {}", data.username)))
    } else {
        Ok(HttpResponse::Ok().body("Password is incorrect."))
    }
}
impl From<argonautica::Error> for ServerError {
    fn from(_: argonautica::Error) -> ServerError {
        ServerError::ArgonauticError
    }
}
async fn user_profile(tera: web::Data<Tera>,
    web::Path(requested_user): web::Path<String>
) -> impl Responder {
    use schema::users::dsl::{username, users};

    let connection = establish_connection();
    let user :User = users.filter(username.eq(requested_user))
        .get_result(&connection)
        .expect("Failed to find user.");

    let posts :Vec<Post> = Post::belonging_to(&user)
        .load(&connection)
        .expect("Failed to find posts.");

    let comments :Vec<Comment> = Comment::belonging_to(&user)
        .load(&connection)
        .expect("Failed to find comments.");

    let mut data = Context::new();
    data.insert("title", &format!("{} - Profile", user.username));
    data.insert("user", &user);
    data.insert("posts", &posts);
    data.insert("comments", &comments);

    let rendered = tera.render("profile.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}
async fn process_signup(data: web::Form<NewUser>) -> impl Responder {
    use schema::users;

    let connection = establish_connection();

    let new_user = NewUser::new(data.username.clone(), data.email.clone(), data.password.clone());

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&connection)
        .expect("Error registering used.");

    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}
async fn process_signup(data: web::Form<NewUser>) -> impl Responder {
    use schema::users;

    let connection = establish_connection();

    diesel::insert_into(users::table)
        .values(&*data)
        .get_result::<User>(&connection)
        .expect("Error registering user.");

    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}
async fn index(tera: web::Data<Tera>, pool: web::Data<Pool>) -> impl Responder {
    use schema::posts::dsl::{posts};
    use schema::users::dsl::{users};

    let connection = pool.get().unwrap();
    let all_posts :Vec<(Post, User)> = posts.inner_join(users)
        .load(&connection)
        .expect("Error retrieving all posts.");

    let mut data = Context::new();
    data.insert("title", "Hacker Clone");
    data.insert("posts_users", &all_posts);

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}
async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().body("Logged out.")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager)
            .expect("Failed to create postgres pool.");

    env_logger::init();

    HttpServer::new(move || {
        let tera = Tera::new("templates/**/*").unwrap();

        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(&[0;32])
                    .name("auth-cookie")
                    .secure(false)
            )
            )
            .data(tera)
            .data(pool.clone())
            .route("/", web::get().to(index))
            .route("/signup", web::get().to(signup))
            .route("/signup", web::post().to(process_signup))
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(process_login))
            .route("/login", web::post().to(process_login))
            .route("/logout", web::to(logout))
            .route("/submission", web::get().to(submission))
            .route("/submission", web::post().to(process_submission))
                
            .service(
                web::resource("/post/{post_id}")
                    .route(web::get().to(post_page))
                    .route(web::post().to(comment))
            )
            .service(
                web::resource("/user/{username}")
                    .route(web::get().to(user_profile))
            )
        })
    .bind("localhost:2000")?
    .run()
    .await
}