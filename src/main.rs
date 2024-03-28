#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::fs::NamedFile;

mod auth;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("views/index.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    let connection = sqlite::open(":memory:").unwrap();
    let query = "CREATE TABLE users (email TEXT, password TEXT);";
    connection.execute(query).unwrap();
    rocket::build()
        .mount("/", routes![index])
        .mount("/auth", routes![auth::signup_get, auth::signup_post])
        .mount("/public", FileServer::from("public"))
}
