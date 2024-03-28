use rocket::form::Form;
use rocket::fs::NamedFile;

#[get("/signup")]
pub async fn signup_get() -> Option<NamedFile> {
    NamedFile::open("views/signup.html").await.ok()
}

#[derive(FromForm, Debug)]
pub struct User<'r> {
    email: &'r str,
    password: &'r str,
}

#[post("/signup", data = "<user>")]
pub async fn signup_post(user: Form<User<'_>>) {
    println!("New user:{:?}", user);
    let connection = sqlite::open(":memory:").unwrap();
    let query = "INSERT INTO users VALUES (:email, :password);";
    let mut statement = connection.prepare(query).unwrap();
    let _ = statement.bind_iter([(":email", user.email), (":password", user.password)]);
    println!("{:?}", connection.execute("SELECT * FROM USERS"))
}
