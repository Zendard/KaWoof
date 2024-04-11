use crate::db_connection;
use crate::UserAuth;
use crate::UserDB;
use cookie::time::Duration;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::{Cookie, CookieJar};

#[derive(FromForm, Debug, PartialEq)]
pub struct UserLogin {
    email: String,
    password: String,
}

#[get("/signup")]
pub async fn signup_get() -> Option<NamedFile> {
    NamedFile::open("views/signup.html").await.ok()
}

#[post("/signup", data = "<user>")]
pub async fn signup_post(user: Form<UserLogin>) -> rocket::response::Redirect {
    println!("New user:{:?}", user);
    let hashed_password = pwhash::bcrypt::hash(&user.password).unwrap();

    let connection = db_connection().await;
    sqlx::query("INSERT INTO users(email,password) VALUES (?,?);")
        .bind(&user.email)
        .bind(hashed_password)
        .execute(&connection)
        .await
        .unwrap();
    rocket::response::Redirect::to("/")
}

async fn get_users() -> Vec<UserDB> {
    let connection = db_connection().await;
    let rows = sqlx::query_as!(UserDB, "SELECT * FROM users;")
        .fetch_all(&connection)
        .await
        .unwrap();
    println!("{:#?}", rows);
    return rows;
}

#[post("/check-user", data = "<user_login>")]
pub async fn check_user(
    user_login: Form<UserLogin>,
    cookies: &CookieJar<'_>,
) -> rocket::response::Redirect {
    let userdb = get_users().await;

    let correct_user_vec: Vec<&UserDB> = userdb
        .iter()
        .filter(|e| {
            e.email == user_login.email && pwhash::bcrypt::verify(&user_login.password, &e.password)
        })
        .collect();

    if correct_user_vec.len() != 1 {
        println!("Wrong password");
        return rocket::response::Redirect::to("login");
    };

    let token = UserAuth::sign(UserAuth {
        id: correct_user_vec[0].id.try_into().unwrap(),
    });
    println!("{:#?}", UserAuth::decode(token.clone()));

    cookies.add(
        Cookie::build(("token", token.clone()))
            .secure(true)
            .max_age(Duration::weeks(5)),
    );
    rocket::response::Redirect::to("/")
}

#[get("/login")]
pub async fn login() -> Option<NamedFile> {
    NamedFile::open("views/login.html").await.ok()
}
