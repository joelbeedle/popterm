const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", {
    name: greetInputEl.value,
  });
}

let outputElement;

async function runCommand() {
  const command = greetInputEl.value;
  if (!command.trim()) {
    outputElement.textContent = "Please enter a command.";
    return;
  }
  outputElement.textContent = "Running...";
  try {
    const output = await invoke("run_command", { command });
    outputElement.textContent = output;
  } catch (err) {
    outputElement.textContent = `Error: ${err}`;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");

  outputElement = document.getElementById("output");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    //greet();
    runCommand();
  });
});
