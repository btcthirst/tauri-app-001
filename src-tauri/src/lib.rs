use tauri_plugin_dialog::DialogExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn help(name: &str) -> String {
    format!("Hello, this is {}! You can update docs!", name.to_ascii_uppercase())
}

#[tauri::command]
fn click1(app: tauri::AppHandle) {
    app.dialog().file().pick_file(|p| println!("{:?}",Some(p)));
}
#[tauri::command]
fn click2(app: tauri::AppHandle) {
    app.dialog().file().pick_file(|p| println!("{:?}",Some(p)));
}
#[tauri::command]
fn click3(app: tauri::AppHandle) {
    app.dialog().file().pick_file(|p| println!("{:?}",Some(p)));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![help,click1,click2,click3])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
