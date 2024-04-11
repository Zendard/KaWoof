const question_element = document.getElementById("question")
const kawoof_id = window.location.pathname.replace("/lobby/", "")

let STATE = {
  connected: false,
  current_question: {}
}

function update_question() {
  question_element.innerText = STATE.current_question.question
}

function connect(uri) {
  const events = new EventSource(uri)

  events.addEventListener("open", (_) => {
    STATE.connected = true
    console.log(`Connected to stream at ${uri}`)
  })

  events.addEventListener("error", (_) => {
    STATE.connected = false
    events.close()

    console.error(`Connection lost, reconnecting...`)
    setTimeout(() => connect(uri), 1000)
  })

  events.addEventListener("next_question", (e) => {
    const json = JSON.parse(e.data)
    console.log(json)

    STATE.current_question = json.question
    update_question()
  })

  events.addEventListener("player_joined", (e) => {
    const json = JSON.parse(e.data)
    console.log(`Player ${json.name} joined`)
  })
}

const uri = "/host/events"
connect(uri)
