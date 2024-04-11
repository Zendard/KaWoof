const question_element = document.getElementById("question")
const answers_element = document.getElementById("answers")
const kawoof_id = window.location.pathname.replace("/lobby/", "")
const reconnecting_dialog = document.getElementById("reconnecting-dialog")

let STATE = {
  connected: false,
  player_id: document.body.dataset.id,
  points: 0,
  current_question: {}
}

function update_question() {
  question_element.innerText = STATE.current_question.question

  const answers = STATE.current_question.answers
  for (let answer_index = 1; answer_index <= answers.length; answer_index++) {
    let button = document.createElement("button")
    button.innerText = answers[answer_index - 1]
    button.classList.add("answer")

    answers_element.appendChild(button)

    button.addEventListener("click", (_) => sendAnswer(answer_index))
  };
}

function sendAnswer(answer_index) {
  const params = {
    answer_id: answer_index,
    question_id: STATE.current_question.id,
    player_id: STATE.player_id,
  }
  console.log(params)
  window.fetch(document.URL + "/post-answer", { method: "post", body: new URLSearchParams(params) })
}

function connect(uri) {
  const events = new EventSource(uri)

  events.addEventListener("open", (_) => {
    STATE.connected = true
    console.log(`Connected to stream at ${uri}`)
    reconnecting_dialog.close()
  })

  events.addEventListener("error", (_) => {
    STATE.connected = false
    events.close()

    console.error(`Connection lost, reconnecting...`)
    reconnecting_dialog.showModal()
    setTimeout(() => { connect(uri) }, 1000)
  })

  events.addEventListener("next_question", (e) => {
    const json = JSON.parse(e.data)
    console.log(json)

    STATE.current_question = json.question
    update_question()
  })
}

const uri = "/host/events"
connect(uri)
