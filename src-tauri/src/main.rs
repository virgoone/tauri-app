#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn main() {
    let menu_instance = menu_instance();

    tauri::Builder::default()
        .menu(menu_instance)
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "close" => {
                event.window().close().unwrap();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn menu_instance() -> Menu {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let close = CustomMenuItem::new("close".to_string(), "关闭");
    let submenu = Submenu::new("文件", Menu::new().add_item(quit).add_item(close));

    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "隐藏"))
        .add_submenu(submenu);

    return menu;
}
