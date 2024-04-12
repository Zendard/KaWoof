const form = document.querySelector("form>.questions")
const question_template = document.querySelector("template").content

const question_counter_init = 0
let question_counter = question_counter_init

function addQuestion() {
  const new_question = question_template.firstElementChild.cloneNode(true)

  new_question.querySelector(".question-input").attributes.name.value = `questions[${question_counter}].question`
  Array.from(new_question.querySelector(".answers").children).forEach(e => {
    e.querySelector("input[type=text]").attributes.name.value = e.querySelector("input[type=text]").attributes.name.value.replace("?", question_counter)
    e.querySelector("input[type=radio]").attributes.name.value = `questions[${question_counter}].correct_answer`
  });

  new_question.querySelector(".question-input").addEventListener("input", inputEventListener)
  form.appendChild(new_question)
}

function inputEventListener(e) {
  e.target.removeEventListener("input", inputEventListener)
  question_counter++
  addQuestion()
}

for (i = 0; i <= question_counter_init; i++) {
  addQuestion()
}

document.querySelector("form").onsubmit = (e) => {
  e.preventDefault()

  let payload = new FormData(document.querySelector('form'));
  [...payload.entries()].forEach(([key, value]) => {
    if (value == 0) payload.delete(key);
  });

  console.log(payload)
  window.fetch("/create", { method: "post", body: payload }).then((e) => {
    if (e.ok) { window.location.href = "/" }
  })
}
