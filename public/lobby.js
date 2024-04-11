let question_element = document.getElementById("question")
let player_list = document.getElementById("players")
let kawoof_id = window.location.pathname.replace("/host/", "")
let next_button = document.getElementById("next_button")

let STATE = {
  connected: false,
  started: false,
  current_question: "",
  players: []
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

  events.addEventListener("player_joined", (e) => {
    const json = JSON.parse(e.data)
    console.log(`Player ${json.name} joined`)
    STATE.players.push(json)

    const listItem = document.createElement("li")
    listItem.innerText = json.name
    player_list.appendChild(listItem)
  })

  events.addEventListener("next_question", (e) => {
    const json = JSON.parse(e.data)
    console.log(json)

    STATE.started = true
    STATE.current_question = json.question
  })
}

next_button.addEventListener("click", (_e) => {
  window.fetch(document.URL + "/next-question", { method: "post" });
})

const uri = "/host/events"
connect(uri)
