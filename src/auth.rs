use rocket::form::Form;
use rocket::fs::NamedFile;

#[get("/signup")]
pub async fn signup_get() -> Option<NamedFile> {
    NamedFile::open("views/signup.html").await.ok()
}

#[derive(FromForm, Debug, PartialEq)]
pub struct User {
    email: String,
    password: String,
}

#[post("/signup", data = "<user>")]
pub async fn signup_post(user: Form<User>) {
    println!("New user:{:?}", user);
    let connection = rusqlite::Connection::open("./db/users.db").unwrap();
    let query = "INSERT INTO users VALUES (:email, :password);";
    let _ = connection.execute(query, (&user.email, &user.password));
}

async fn get_users() -> Vec<User> {
    let connection = rusqlite::Connection::open("./db/users.db").unwrap();
    let query = "SELECT email,password FROM users";
    let rows = connection
        .prepare(query)
        .unwrap()
        .query_map([], |row| {
            Ok(User {
                email: row.get(0)?,
                password: row.get(1)?,
            })
        })
        .unwrap()
        .map(|row| row.unwrap())
        .collect();
    return rows;
}

#[post("/check-user", data = "<user>")]
pub async fn check_user(user: Form<User>) -> String {
    let userdb = get_users().await;
    if !userdb.contains(&user) {
        return "Wrong password".to_string();
    } else {
        return "Logged in!".to_string();
    }
}

#[get("/login")]
pub async fn login() -> Option<NamedFile> {
    NamedFile::open("views/login.html").await.ok()
}
