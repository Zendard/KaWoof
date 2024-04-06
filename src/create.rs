use crate::UserAuth;
use rocket::form::Form;
use rocket::fs::NamedFile;

#[get("/")]
pub async fn create(_user: UserAuth) -> Option<NamedFile> {
    NamedFile::open("views/create.html").await.ok()
}

#[derive(FromForm, std::fmt::Debug)]
pub struct KaWoof {
    title: String,
    questions: Vec<Question>,
}

#[derive(FromForm, std::fmt::Debug)]
struct Question {
    question: String,
    answers: Vec<String>,
    correct_answer: u8,
}

#[post("/", data = "<kawoof>")]
pub async fn create_post(user: UserAuth, kawoof: Form<KaWoof>) -> rocket::response::Redirect {
    let connection = rusqlite::Connection::open("./db/kawoof.db").unwrap();
    let query = "INSERT INTO kawoofs(title,author,questions) VALUES (:title,:author,:questions);";
    let exec = connection.execute(
        query,
        (
            kawoof.title.clone(),
            user.id,
            kawoof
                .questions
                .iter()
                .map(|e| e.question.clone())
                .collect::<Vec<String>>()
                .join("|"),
        ),
    );
    println!("{:#?}", exec);

    println!("{:#?}", kawoof);
    rocket::response::Redirect::to("/")
}
