let playerList = document.getElementById("players")
let kawoof_id = window.location.pathname.replace("/host/", "")
let next_button = document.getElementById("next_button")
console.log(kawoof_id)

const events = new EventSource(document.URL + "/events")

next_button.addEventListener("click", (_e) => {
  window.fetch(document.URL + "/next-question", { method: "post" });

})

events.addEventListener("player_joined", (e) => {
  console.log("Player joined!")
  console.log(e.data)
  const listItem = document.createElement("li")
  listItem.innerText = JSON.parse(e.data).name
  playerList.appendChild(listItem)
})

events.addEventListener("next_question", (e) => {
  console.log(JSON.parse(e.data));
})
