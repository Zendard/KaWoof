use crate::UserAuth;
use cookie::time::Duration;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::{Cookie, CookieJar};

#[derive(FromForm, Debug, PartialEq)]
pub struct UserDB {
    id: u32,
    email: String,
    password: String,
}
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
pub async fn signup_post(user: Form<UserLogin>) -> String {
    println!("New user:{:?}", user);
    let connection = rusqlite::Connection::open("./db/users.db").unwrap();
    let query = "INSERT INTO users(email,password) VALUES (:email, :password);";
    let exec = connection.execute(query, (&user.email, &user.password));
    println!("{:#?}", exec);
    return "Tjoem".to_string();
}

async fn get_users() -> Vec<UserDB> {
    let connection = rusqlite::Connection::open("./db/users.db").unwrap();
    let query = "SELECT * FROM users";
    let rows = connection
        .prepare(query)
        .unwrap()
        .query_map([], |row| {
            Ok(UserDB {
                id: row.get(0)?,
                email: row.get(1)?,
                password: row.get(2)?,
            })
        })
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    println!("{:#?}", rows);
    return rows;
}

#[post("/check-user", data = "<user_login>")]
pub async fn check_user(user_login: Form<UserLogin>, cookies: &CookieJar<'_>) -> Option<String> {
    let userdb = get_users().await;

    let correct_user_vec: Vec<&UserDB> = userdb
        .iter()
        .filter(|e| e.email == user_login.email && e.password == user_login.password)
        .collect();

    if correct_user_vec.len() != 1 {
        println!("Wrong password");
        return None;
    }

    let token = UserAuth::sign(UserAuth {
        id: correct_user_vec[0].id,
    });
    println!("{:#?}", UserAuth::decode(token.clone()));

    cookies.add(
        Cookie::build(("token", token.clone()))
            .secure(true)
            .max_age(Duration::weeks(5)),
    );
    return Some(token);
}

#[get("/login")]
pub async fn login() -> Option<NamedFile> {
    NamedFile::open("views/login.html").await.ok()
}
