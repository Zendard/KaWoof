use crate::{query_kawoof, ClientQuestion, NextQuestionEvent};
use crate::{HostEvent, UserAuth};
use rocket::response::stream::{Event, EventStream};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
use rocket::{Shutdown, State};
use rocket_dyn_templates::{context, Template};

#[get("/<kawoof_id>")]
pub async fn host_kawoof(user: UserAuth, kawoof_id: u32) -> Template {
    let kawoof = crate::query_kawoof(kawoof_id, Some(&user)).await;
    Template::render("host_kawoof", context! {kawoof})
}

#[get("/events")]
pub async fn stream<'a>(queue: &State<Sender<HostEvent>>, mut end: Shutdown) -> EventStream![] {
    let mut receiver = queue.subscribe();

    EventStream! {
    loop {
        let msg: HostEvent = select! {
        msg = receiver.recv() => match msg {
                 Ok(msg) => {msg},
                 Err(RecvError::Closed) => break,
                 Err(RecvError::Lagged(_)) => continue,
             },
         _=&mut end =>break
         };
        match msg {
            HostEvent::PlayerJoined(msg) => yield Event::json(&msg).event("player_joined"),
            HostEvent::NextQuestion(msg) => yield Event::json(&msg).event("next_question"),
            HostEvent::Answer(msg) => yield Event::json(&msg).event("answer"),
        };
        }
    }
}

#[post("/<kawoof_id>/next-question", data = "<question_counter>")]
pub async fn next_question(
    user: UserAuth,
    kawoof_id: u32,
    queue: &State<Sender<HostEvent>>,
    question_counter: rocket::form::Form<usize>,
) -> Option<String> {
    let kawoof = query_kawoof(kawoof_id, Some(&user)).await;
    let question_counter = question_counter.into_inner();

    let question = ClientQuestion {
        id: kawoof.questions.get(question_counter)?.id,
        question: kawoof.questions.get(question_counter)?.question.clone(),
        answers: kawoof.questions.get(question_counter)?.answers.clone(),
    };

    queue
        .send(HostEvent::NextQuestion(NextQuestionEvent {
            kawoof_id,
            question,
        }))
        .unwrap();

    Some("Next question".to_string())
}
