#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::fs::NamedFile;
use serde::Serialize;
mod auth;
mod create;
mod get_kawoofs;

#[get("/")]
async fn index(user: UserAuth) -> Option<NamedFile> {
    println!("{:#?}", user);
    NamedFile::open("views/index.html").await.ok()
}

#[catch(401)]
async fn no_account() -> Option<NamedFile> {
    println!("Not logged in");
    NamedFile::open("views/no_account.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![no_account])
        .mount(
            "/auth",
            routes![
                auth::signup_get,
                auth::signup_post,
                auth::login,
                auth::check_user,
            ],
        )
        .mount("/create", routes![create::create, create::create_post])
        .mount("/my-kawoofs", routes![get_kawoofs::get_kawoofs])
        .mount("/public", FileServer::from("public"))
        .attach(rocket_dyn_templates::Template::fairing())
}

//Exports:

#[macro_export]
macro_rules! db_connection {
    () => {
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect("./kawoof.db")
            .await
            .unwrap()
    };
}
//Authentication/users
static SECRET_KEY: &str = env!("SECRET_KEY");

#[rocket_jwt_new::jwt(SECRET_KEY, cookie = "token")]
pub struct UserAuth {
    id: i64,
}

#[derive(FromForm, Debug, PartialEq, sqlx::FromRow)]
pub struct UserDB {
    id: i64,
    email: String,
    password: String,
}
//Kawoofs
#[derive(std::fmt::Debug, Serialize)]
pub struct KaWoof {
    id: i64,
    title: String,
    author: i64,
    questions: Vec<Question>,
}

#[derive(FromForm, std::fmt::Debug, Serialize)]
struct Question {
    question: String,
    answers: Vec<String>,
    correct_answer: i64,
}
