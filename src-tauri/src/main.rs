#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

static VERSION: &str = "0.1.2";

// Change the version in following files:
//  - /package.json
//  - /src-tauri/Cargo.toml
//  - /src-tauri/tauri.conf.json

extern crate winreg;

use std::env;
use std::fs;
use std::io;
use std::path::Path;

use tauri::Manager;

//use tauri::LogicalSize;
use tauri::api::dialog;

use winreg::enums::*;
use winreg::RegKey;

fn main() {
    println!("{}", VERSION);
    
    tauri::Builder::default()
        .setup(|app| {
            
            let args: Vec<String> = env::args().collect();
            _ = register_as_default_app(&args[0]);
            
            let main = app.get_window("main").unwrap();
            let splash = app.get_window("splash").unwrap();
            
            if args.len() >= 2 {
                
                main.center().unwrap();
                main.show().unwrap();
                
                splash.close().unwrap();
                
                tauri::async_runtime::spawn(async move {
                    let contents = fs::read_to_string(&args[1]).expect("OK");
                    main.emit("onLoadTauslyCodeFile", contents).unwrap();
                });
                
            } else {
                
                splash.center().unwrap();
                splash.show().unwrap();
                
                main.close().unwrap();
                
                tauri::async_runtime::spawn(async move {
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    splash.close().unwrap();
                });
                
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_file_dialog,
            resize_window
        ])
        //.on_window_event(|event| match event.event() {
        //    tauri::WindowEvent::Resized(size) => {
        //        let window = event.window();
        //        let s = window.scale_factor().unwrap();
        //        let l_size: LogicalSize<u32> = size.to_logical(s);
        //        window.emit("onWindowResize", l_size).unwrap();
        //    }
        //    _ => { }
        //})
        .run(tauri::generate_context!())
        .expect("ERROR");
}

#[tauri::command]
fn open_file_dialog(window: tauri::Window) {
    dialog::FileDialogBuilder::default()
        .add_filter("Tausly Code File", &["tausly"])
        .pick_file(move |path_buf| match path_buf {
            Some(path) => {
                let contents = fs::read_to_string(path).expect("OK");
                window.emit("onLoadTauslyCodeFile", contents).unwrap();
            }
            _ => { }
        });
}

#[tauri::command]
fn resize_window(window: tauri::Window, width: f64, height: f64) {
    window.set_size(
        tauri::Size::Logical(
            tauri::LogicalSize {
                width,
                height
            }
        )
    ).unwrap();
    window.center().unwrap();
}

fn register_as_default_app(executable_path: &str) -> io::Result<()> {
    
    let path = Path::new("SOFTWARE").join("Classes");
    let path_ext = path.join(".tausly");
    let path_name = path.join("TauslyCodeFile");
    let path_command = path_name.join("shell").join("open").join("command");
    let path_icon = path_name.join("DefaultIcon");
    
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    let (path_ext_key, _) = hkcu.create_subkey(&path_ext)?;
    path_ext_key.set_value("", &"TauslyCodeFile")?;
    
    let (path_name_key, _) = hkcu.create_subkey(&path_name)?;
    path_name_key.set_value("", &"Tausly Code File")?;
    
    let (path_command_key, _) = hkcu.create_subkey(&path_command)?;
    let command = format!("{}{}{}", &"\"", executable_path, &"\" \"%1\"");
    path_command_key.set_value("", &command)?;
    
    let (path_icon_key, _) = hkcu.create_subkey(&path_icon)?;
    let icon = format!("{}{}{}", &"\"", executable_path, &"\"");
    path_icon_key.set_value("", &icon)?;
    
    Ok(())
}