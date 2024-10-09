// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::tray::TrayIconBuilder;

fn main() {
  tauri::Builder::default()
      .on_menu_event(|_app, event| {
          println!("menu event from global handler: {:?}", event);
      })

      .setup(|app| {
          let tray_menu = tauri::menu::MenuBuilder::new(app.handle())
              .text("hi", "Hi")
              .build()?;


          TrayIconBuilder::new()
              .icon(app.default_window_icon().unwrap().clone())
              .icon_as_template(true)
              .menu(&tray_menu)
              .on_menu_event(|_, event| {
                  println!("menu event from tray handler: {:?}", event);
              })
              .build(app)?;

          Ok(())
      })
      .run(tauri::generate_context!(
      "../../examples/tray/tauri.conf.json"
    ))
      .expect("error while running tauri application");
}
