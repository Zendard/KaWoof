use rocket::tokio::sync::broadcast::Sender;

#[derive(FromForm, Debug)]
pub struct CheckAnswerForm {
    answer_id: i64,
    question_id: u32,
    player_id: u32,
}

pub struct CorrectAnswerDB {
    correct_answer: i64,
}

#[post("/<kawoof_id>/post-answer", data = "<data>")]
pub async fn check_answer(
    kawoof_id: u32,
    data: rocket::form::Form<CheckAnswerForm>,
    queue: &rocket::State<Sender<crate::HostEvent>>,
) {
    let correct_answer = sqlx::query_as!(
        CorrectAnswerDB,
        "SELECT correct_answer FROM questions WHERE id=?",
        data.question_id
    )
    .fetch_one(&crate::db_connection().await)
    .await
    .unwrap()
    .correct_answer;

    println!("{:?}", data.answer_id);
    println!("{:?}", correct_answer);
    let correct = data.answer_id == correct_answer;

    queue
        .send(crate::HostEvent::Answer(crate::AnswerEvent {
            correct,
            player_id: data.player_id,
            kawoof_id,
        }))
        .unwrap();
}
