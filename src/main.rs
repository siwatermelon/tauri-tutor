#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// use std::fs;

#[tauri::command]
fn greet(url: String) -> String {
  // let output = "rust.md";
  let body = reqwest::blocking::get(url)
  .unwrap()
  .text().unwrap();
  println!("Converting response to markdown...");
  let md = html2md::parse_html(&body);
  // fs::write(output, md.as_bytes()).unwrap();
  md
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
