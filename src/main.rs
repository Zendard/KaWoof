#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::fs::NamedFile;
mod auth;
mod create;

#[get("/")]
async fn index(user: UserAuth) -> Option<NamedFile> {
    println!("{:#?}", user);
    NamedFile::open("views/index.html").await.ok()
}

#[get("/", rank = 2)]
async fn no_account() -> Option<NamedFile> {
    NamedFile::open("views/no_account.html").await.ok()
}

static SECRET_KEY: &str = env!("SECRET_KEY");

#[rocket_jwt_new::jwt(SECRET_KEY, cookie = "token")]
pub struct UserAuth {
    id: u32,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, no_account])
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
        .mount("/public", FileServer::from("public"))
}
