use crate::Player;
use crate::UserAuth;
use rocket::response::stream::{Event, EventStream};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
use rocket::State;
use rocket_dyn_templates::{context, Template};

#[get("/<kawoof_id>")]
pub async fn host_kawoof(user: UserAuth, kawoof_id: i64) -> Template {
    let kawoof = crate::query_kawoof(kawoof_id, &user).await;

    Template::render("host_kawoof", context! {kawoof})
}

#[get("/<kawoof_id>/events")]
pub async fn stream<'a>(
    kawoof_id: i64,
    user: UserAuth,
    player_join_queue: &State<Sender<Player>>,
    question_id_queue: &State<Sender<u32>>,
) -> EventStream![Event + 'a] {
    let mut receiver_player_join = player_join_queue.subscribe();
    let mut receiver_question_id = question_id_queue.subscribe();
    let kawoof = crate::query_kawoof(kawoof_id, &user).await;
    let mut question_counter = 0;
    EventStream! {
    loop{
        let player = select! {
           msg = receiver_player_join.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError ::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
            };
           yield Event::json(&player).event("player_joined");

        let question = select! {
           msg = receiver_question_id.recv() => match msg {
                    Ok(_msg) => {question_counter+=1;&kawoof.questions[question_counter-1]},
                    Err(RecvError ::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
            };
            yield Event::json(&question).event("next_question");
    }
    }
}

#[post("/<_kawoof_id>/next-question")]
pub async fn next_question(
    _user: UserAuth,
    _kawoof_id: i64,
    question_id_queue: &State<Sender<u32>>,
) {
    question_id_queue.send(1).unwrap();
}
