// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_handler, AppHandle, Manager};

#[tauri::command]
fn get_fs_allowed_patterns(app_handle: AppHandle) -> Vec<String> {
  let patterns = app_handle.fs_scope().allowed_patterns();
  return patterns.iter().map(|e| e.to_string()).collect::<Vec<_>>();
}

#[tauri::command]
async fn resolve_path(
  resolver: tauri::State<'_, tauri_plugin_extended_fs_scope::PatternResolver>,
  pattern: &str,
) -> Result<String, tauri_plugin_extended_fs_scope::ResolveError> {
  let result = resolver.resolve(pattern).map(|e| e.to_string())?;
  Ok(result)
}

#[tauri::command]
fn resolver_variables(
  resolver: tauri::State<'_, tauri_plugin_extended_fs_scope::PatternResolver>,
) -> Vec<String> {
  resolver.variables().map(|e| e.0.to_string()).collect()
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_extended_fs_scope::init())
    .invoke_handler(generate_handler![
      get_fs_allowed_patterns,
      resolve_path,
      resolver_variables
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
