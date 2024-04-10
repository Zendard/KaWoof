use crate::db_connection;
use crate::Question;
use crate::UserAuth;
use rocket::form::Form;
use rocket::fs::NamedFile;

#[get("/")]
pub async fn create(_user: UserAuth) -> Option<NamedFile> {
    NamedFile::open("views/create.html").await.ok()
}

#[derive(FromForm)]
pub struct KaWoofForm {
    title: String,
    questions: Vec<Question>,
}

#[post("/", data = "<kawoof>")]
pub async fn create_post(user: UserAuth, kawoof: Form<KaWoofForm>) -> rocket::response::Redirect {
    let connection = db_connection().await;

    let mut question_ids: Vec<i64> = vec![];
    println!("{:#?}", kawoof.questions);
    for question in kawoof.questions.iter() {
        let answers_joined = question
            .answers
            .iter()
            .map(|e| e.replace(";", ","))
            .collect::<Vec<String>>()
            .join(";");

        let last_id = sqlx::query!(
            "INSERT INTO questions(question,correct_answer,answers) VALUES (?,?,?)",
            question.question,
            question.correct_answer,
            answers_joined
        )
        .execute(&connection)
        .await
        .unwrap()
        .last_insert_rowid();
        question_ids.push(last_id);
    }
    let question_ids: Vec<u8> = question_ids.iter().map(|e| *e as u8).collect();
    println!("{:#?}", user);
    println!("{:#?}", question_ids);

    sqlx::query!(
        "INSERT INTO kawoofs(title,author,questions) VALUES (?,?,?)",
        kawoof.title,
        user.id,
        question_ids
    )
    .execute(&connection)
    .await
    .unwrap();

    rocket::response::Redirect::to("/")
}
