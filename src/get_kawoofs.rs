use crate::db_connection;
use crate::query_kawoof;
use crate::KaWoof;
use crate::KawoofDB;
use crate::Question;
use crate::QuestionDB;
use crate::UserAuth;
use rocket_dyn_templates::{context, Template};

#[get("/<id>")]
pub async fn kawoof_details(user: UserAuth, id: u32) -> Template {
    let kawoof = query_kawoof(id, Some(&user)).await;

    Template::render("kawoof_details", context! {user, kawoof})
}

#[get("/")]
pub async fn get_kawoofs(user: UserAuth) -> Template {
    let connection = db_connection().await;

    let kawoofs_raw = sqlx::query_as!(KawoofDB, "SELECT * FROM kawoofs WHERE author=?", user.id)
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
                correct_answer: question_raw.correct_answer.try_into().unwrap(),
                answers,
            })
        }
        kawoofs.push(KaWoof {
            id: kawoof.id.try_into().unwrap(),
            title: kawoof.title.clone(),
            author: kawoof.author.try_into().unwrap(),
            questions,
        })
    }
    println!("{:#?}", kawoofs);
    Template::render("my_kawoofs", context! { kawoofs,user })
}
