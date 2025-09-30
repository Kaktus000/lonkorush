#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // hole Referenz zum Hauptfenster ("main" aus tauri.conf.json)
            let window = app.get_webview_window("main").unwrap();

            // starte async Task für URL-Fetch
            tauri::async_runtime::spawn(async move {
                let url_file = "https://raw.githubusercontent.com/Kaktus000/lonkorush/refs/heads/main/ip.txt";

                match reqwest::get(url_file).await {
                    Ok(resp) => {
                        if let Ok(text) = resp.text().await {
                            let ip_url = text.trim().to_string();
                            // leite Window um
                            let _ = window.eval(&format!(
                                "window.location.replace('{}')",
                                ip_url
                            ));
                        }
                    }
                    Err(err) => {
                        eprintln!("Fehler beim Laden der URL-Datei: {}", err);
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
