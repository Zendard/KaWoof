const question_element = document.getElementById("question")
const player_list = document.getElementById("players")
const kawoof_id = window.location.pathname.replace("/host/", "")
const next_button = document.getElementById("next_button")
const reconnecting_dialog = document.getElementById("reconnecting-dialog")
const player_points_list = document.getElementById("player-list")

let STATE = {
  connected: false,
  started: false,
  question_counter: 0,
  current_question: {},
  players: [],
  received_answers: [],
}

function update_question() {
  question_element.innerText = STATE.current_question.question
  next_button.style.display = "none"
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

    if (!STATE.started) {
      document.getElementById("pre-start").remove()
      next_button.innerText = "Next"
    }

    STATE.started = true
    STATE.current_question = json.question
    update_question()

    events.addEventListener("answer", answerReceiver)

  })
}

function answerReceiver(e) {
  const events = e.target
  const json = JSON.parse(e.data)

  if (!STATE.players.map((e) => e.id).includes(json.player_id)) { console.log("not in player list"); return }
  if (STATE.received_answers.map((e) => e.player_id).includes(json.player_id)) return

  STATE.received_answers.push(json)

  if (STATE.received_answers.length >= STATE.players.length) {
    events.removeEventListener("answer", answerReceiver)
    tallyPoints()
  }
}

function tallyPoints() {
  console.log(STATE.received_answers)
  let player_list = []
  STATE.players.sort((a, b) => a.points - b.points)
  STATE.players.forEach(player => {
    if (STATE.received_answers.filter((e) => e.player_id == player.id)[0].correct) {
      player.points += 100
    }
    const list_element = document.createElement("li")
    list_element.innerText = `${player.name}: ${player.points}`
    player_list.push(list_element)
  });

  player_points_list.style.display = "block"
  player_list.forEach((e) => {
    player_points_list.appendChild(e)
  })

  next_button.style.display = "block"
}

function finish() {
  question_element.style.display = "none"
  player_points_list.style.fontSize = "3rem"
  next_button.innerText = "Home"
  next_button.addEventListener("click", (_) => window.location.href = "/")
}

next_button.addEventListener("click", (_e) => {
  if (!STATE.connected) return
  const question_counter = STATE.question_counter
  window.fetch(document.URL + "/next-question", { method: "post", body: new URLSearchParams({ question_counter }) }).then((e) => {
    if (e.ok) {
      STATE.question_counter++
      STATE.received_answers = []
      player_points_list.style.display = "none"
      Array.from(player_points_list.children).forEach((e) => e.remove())
    }
    else {
      finish()
    }
  });
})

const uri = "/host/events"
connect(uri)
