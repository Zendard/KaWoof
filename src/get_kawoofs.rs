use crate::db_connection;
use crate::KaWoof;
use crate::Question;
use crate::UserAuth;
use rocket_dyn_templates::{context, Template};
use sqlx::sqlite::SqlitePoolOptions;

#[derive(Debug)]
struct KawoofDB {
    id: i64,
    title: String,
    author: i64,
    questions: Vec<u8>,
}

struct QuestionDB {
    question: String,
    correct_answer: i64,
    answers: String,
}

#[get("/")]
pub async fn get_kawoofs(user: UserAuth) -> Template {
    let connection = db_connection!();
    let kawoofs_raw = sqlx::query_as!(
        KawoofDB,
        "SELECT id,title,author,questions FROM kawoofs WHERE author=?",
        user.id
    )
    .fetch_all(&connection)
    .await
    .unwrap();
    println!("{:#?}", kawoofs_raw);
    let mut kawoofs: Vec<KaWoof> = vec![];

    for kawoof in kawoofs_raw.iter() {
        let mut questions: Vec<Question> = vec![];
        for question_id in &kawoof.questions {
            let question_raw = sqlx::query_as!(
                QuestionDB,
                "SELECT question,correct_answer,answers FROM questions WHERE id=?",
                question_id
            )
            .fetch_one(&connection)
            .await
            .unwrap();
            let answers: Vec<String> = question_raw
                .answers
                .split(";")
                .map(|e| e.to_string())
                .collect();
            questions.push(Question {
                question: question_raw.question,
                correct_answer: question_raw.correct_answer,
                answers,
            })
        }
        kawoofs.push(KaWoof {
            id: kawoof.id,
            title: kawoof.title.clone(),
            author: kawoof.author,
            questions,
        })
    }
    println!("{:#?}", kawoofs);
    Template::render("my_kawoofs", context! { kawoofs,user })
}
