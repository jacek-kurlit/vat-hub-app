mod contractors;
mod db;

pub use db::*;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle();
            let database =
                tauri::async_runtime::block_on(async { db::Database::new(handle).await })?;
            let client = reqwest::Client::new();

            let contractor_service = contractors::ContractorService::new(database.clone(), client);

            app.manage(contractor_service);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            contractors::fetch_contractor_data,
            contractors::save_contractor
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
