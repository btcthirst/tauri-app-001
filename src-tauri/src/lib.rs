use tauri::{command, AppHandle};
use tauri_plugin_dialog::{DialogExt, FilePath};
pub mod data_types;
pub mod calamine_simp;
pub mod xlsx_writer;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[command]
fn help(name: &str) -> String {
    format!("Hello, this is {}! You can update docs!", name.to_ascii_uppercase())
}

#[command]
async  fn click1(app: AppHandle) {
   let _ = &app.dialog()
    .file()
    .set_title("Open file with name 'нарахування'")
    .add_filter("Excel files", &["xlsx"])
    .pick_file(|file_path| {
     match file_path {
         Some(path)=> {
            println!("{:?}",path.clone());
            match calamine_simp::read(path.to_string()){
                Ok(data) => {
                    open(app,data)
                   
                },
                Err(err) => {
                    println!("Err: {:?}", err)
                }
            };
         },
         None =>{
            println!("file path None")
         }
     }

    });
}
#[command]
fn click2(app: AppHandle) {
    app.dialog().file().pick_file(|p| println!("{:?}",Some(p)));
}
#[command]
fn click3(app: AppHandle) {
    app.dialog().file().pick_file(|p| println!("{:?}",Some(p)));
}


fn open(app: AppHandle, data: Vec<Vec<Vec<String>>>) {
    let file_path = app.dialog().file().add_filter("xlsx", &["xlsx"])
    .blocking_save_file().unwrap();
    _= xlsx_writer::write(data,&file_path.to_string());
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
