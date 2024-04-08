use crate::Question;
use crate::UserAuth;
use crate::KaWoof;
use sqlx::sqlite::SqlitePoolOptions;
use crate::db_connection;

#[derive(Debug)]
struct KawoofDB{
    title:String,
    author:i64,
    questions:Vec<u8>
}

struct QuestionDB{
    question: String,
    correct_answer: i64,
    answers: String
}

#[get("/")]
pub async fn get_kawoofs(user:UserAuth) -> String{
let connection = db_connection!();
    let kawoofs_raw = sqlx::query_as!(KawoofDB,"SELECT title,author,questions FROM kawoofs WHERE author=?",user.id).fetch_all(&connection).await.unwrap();
    println!("{:#?}",kawoofs_raw);
    let mut kawoofs:Vec<KaWoof> = vec![];

    for kawoof in kawoofs_raw.iter() {
        let mut questions:Vec<Question> = vec![];
        for question_id in &kawoof.questions{
            let question_raw = sqlx::query_as!(QuestionDB,"SELECT question,correct_answer,answers FROM questions WHERE id=?",question_id).fetch_one(&connection).await.unwrap();
            let answers:Vec<String> = question_raw.answers.split(";").map(|e| e.to_string()).collect();
            questions.push(Question{
                question:question_raw.question,
                correct_answer:question_raw.correct_answer,
                answers
            })
        }
        kawoofs.push(KaWoof{
            title: kawoof.title.clone(),
            author: kawoof.author,
            questions
        })
    }
    println!("{:#?}", kawoofs);

    user.id.to_string()
}
