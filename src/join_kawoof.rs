use crate::{query_kawoof, Player};
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::tokio::sync::broadcast::Sender;
use rocket::State;
use rocket_dyn_templates::{context, Template};

#[get("/join")]
pub async fn enter_id() -> Option<NamedFile> {
    NamedFile::open("./views/enter_code.html").await.ok()
}

#[derive(rocket::serde::Serialize)]
struct ClientKawoof {
    title: String,
    author: u32,
}

#[derive(FromForm)]
pub struct Username {
    name: String,
}

#[post("/lobby/<kawoof_id>", data = "<player>")]
pub async fn join(
    kawoof_id: u32,
    player: Form<Username>,
    queue: &State<Sender<crate::HostEvent>>,
) -> Template {
    queue
        .send(crate::HostEvent::PlayerJoined(Player {
            kawoof_id,
            name: player.name.clone(),
        }))
        .unwrap();

    let kawoof = query_kawoof(kawoof_id, None).await;
    let client_kawoof = ClientKawoof {
        title: kawoof.title,
        author: kawoof.author,
    };

    Template::render("client_play", context! {client_kawoof})
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
