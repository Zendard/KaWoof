#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::fs::NamedFile;
use serde::Serialize;

mod auth;
mod create;
mod get_kawoofs;
mod host_kawoof;

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
        .mount(
            "/my-kawoofs",
            routes![get_kawoofs::get_kawoofs, get_kawoofs::kawoof_details],
        )
        .mount("/host", routes![host_kawoof::host_kawoof])
        .mount("/public", FileServer::from("public"))
        .attach(rocket_dyn_templates::Template::fairing())
}

//Exports:

pub async fn db_connection() -> sqlx::Pool<sqlx::Sqlite> {
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect("./kawoof.db")
        .await
        .unwrap()
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

#[derive(Debug)]
struct KawoofDB {
    id: i64,
    title: String,
    author: i64,
    questions: Vec<u8>,
}

struct QuestionDB {
    question: String,
    correct_answer: i64,
    answers: String,
}

pub async fn query_kawoof(id: i64, user: &UserAuth) -> KaWoof {
    let connection = db_connection().await;

    let kawoof_raw = sqlx::query_as!(
        KawoofDB,
        "SELECT * FROM kawoofs WHERE id=? AND author=?",
        id,
        user.id
    )
    .fetch_one(&connection)
    .await
    .unwrap();

    let mut questions: Vec<Question> = vec![];

    for question_id in kawoof_raw.questions.iter() {
        let question_raw = sqlx::query_as!(
            QuestionDB,
            "SELECT question,correct_answer,answers FROM questions WHERE id=?",
            question_id
        )
        .fetch_one(&connection)
        .await
        .unwrap();

        let answers: Vec<String> = question_raw
            .answers
            .split(";")
            .map(|e| e.to_string())
            .collect();

        questions.push(Question {
            question: question_raw.question,
            correct_answer: question_raw.correct_answer,
            answers,
        });
    }

    KaWoof {
        id: kawoof_raw.id,
        title: kawoof_raw.title,
        author: kawoof_raw.id,
        questions,
    }
}
