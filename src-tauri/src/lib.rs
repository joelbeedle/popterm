use std::process::Command;

#[tauri::command]
fn run_command(command: String) -> Result<String, String> {
  println!("Running command: {}", command); // Debug log for the command being executed

  let output = Command::new("sh").arg("-c").arg(command).output();

  match output {
    Ok(output) => {
      if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
      } else {
        // Include the error output for failed commands
        Err(format!(
          "Command failed with status: {}\nError: {}",
          output.status,
          String::from_utf8_lossy(&output.stderr)
        ))
      }
    }
    Err(err) => {
      // Include a detailed error message for command execution failures
      Err(format!("Failed to execute command: {}", err))
    }
  }
}

#[tauri::command]
fn exit_app() {
  std::process::exit(0x0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![run_command, exit_app])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
