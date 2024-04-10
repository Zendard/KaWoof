use crate::Player;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::{Cookie, CookieJar};
use rocket::tokio::sync::broadcast::Sender;
use rocket::State;
use rocket_dyn_templates::{context, Template};

#[get("/join")]
pub async fn enter_id() -> Option<NamedFile> {
    NamedFile::open("./views/enter_code.html").await.ok()
}

#[derive(rocket::serde::Serialize)]
struct KawoofTitle {
    title: String,
}

#[post("/lobby/<kawoof_id>", data = "<player>")]
pub async fn join(
    kawoof_id: i64,
    player: Form<Player>,
    queue: &State<Sender<Player>>,
    cookie_jar: &CookieJar<'_>,
) -> Template {
    // if cookie_jar.get("rejoin").is_none() {
    queue
        .send(Player {
            name: player.name.clone(),
        })
        .unwrap();
    // cookie_jar.add(Cookie::new("rejoin", "true"))
    // }
    let kawoof_title = sqlx::query_as!(
        KawoofTitle,
        "SELECT title FROM kawoofs WHERE id=?",
        kawoof_id
    )
    .fetch_one(&crate::db_connection().await)
    .await
    .unwrap()
    .title;

    Template::render("client_play", context! {kawoof_title})
}

#[derive(FromForm)]
pub struct RedirectForm {
    kawoof_id: i64,
}

#[post("/join/redirect", data = "<form>")]
pub async fn redirect_to_kawoof_id(form: Form<RedirectForm>) -> rocket::response::Redirect {
    rocket::response::Redirect::to(uri!(get_player_name(form.kawoof_id)))
}

#[get("/join/<kawoof_id>")]
pub async fn get_player_name(kawoof_id: i64) -> Template {
    Template::render("get_player_name", context! {kawoof_id})
}
