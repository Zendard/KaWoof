const question_element = document.getElementById("question")
const player_list = document.getElementById("players")
const kawoof_id = window.location.pathname.replace("/host/", "")
const next_button = document.getElementById("next_button")
const reconnecting_dialog = document.getElementById("reconnecting-dialog")

let STATE = {
  connected: false,
  started: false,
  question_counter: 0,
  current_question: {},
  players: []
}

function update_question() {
  question_element.innerText = STATE.current_question.question
  next_button.style.visibility = "false"
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
    setTimeout(() => connect(uri), 1000)
  })

  events.addEventListener("player_joined", (e) => {
    const json = JSON.parse(e.data)
    console.log(`Player ${json.name} with id ${json.id} joined`)
    STATE.players.push(json)

    const listItem = document.createElement("li")
    listItem.innerText = json.name
    player_list.appendChild(listItem)
  })

  events.addEventListener("next_question", (e) => {
    const json = JSON.parse(e.data)
    console.log(json)

    if (!STATE.started) {
      document.getElementById("pre-start").remove()
    }

    STATE.started = true
    STATE.current_question = json.question
    update_question()

    events.addEventListener("answer", (e) => {
      const json = JSON.parse(e.data)
      console.log(json)
    })

  })
}

next_button.addEventListener("click", (_e) => {
  if (!STATE.connected) return
  const question_counter = STATE.question_counter
  window.fetch(document.URL + "/next-question", { method: "post", body: new URLSearchParams({ question_counter }) });
  STATE.question_counter++
})

const uri = "/host/events"
connect(uri)
