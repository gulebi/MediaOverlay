use serde::Serialize;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

use std::thread;
use std::time::Duration;
use tauri::{Manager, Window};

fn get_properties(
) -> windows::Media::Control::GlobalSystemMediaTransportControlsSessionMediaProperties {
    let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .unwrap()
        .get()
        .unwrap();
    let session = manager.GetCurrentSession().unwrap();
    session.TryGetMediaPropertiesAsync().unwrap().get().unwrap()
}

#[derive(Debug, Serialize)]
struct NowPlaying {
    artist: String,
    title: String,
}

#[tauri::command]
fn get_now_playing(window: Window) -> NowPlaying {
    let properties = get_properties();

    let artist = properties.Artist().unwrap().to_string();
    let title = properties.Title().unwrap().to_string();

    window.show().unwrap();

    let window_clone = window.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        window_clone.hide().unwrap();
    });

    NowPlaying { artist, title }
}

#[tauri::command]
async fn get_thumbnail() -> String {
    let properties = get_properties();

    let thumbnail = properties.Thumbnail().unwrap();
    let stream = thumbnail.OpenReadAsync().unwrap().get().unwrap();
    let size = stream.Size().unwrap() as usize;
    let reader = windows::Storage::Streams::DataReader::CreateDataReader(&stream).unwrap();
    reader.LoadAsync(size as u32).unwrap().get().unwrap();

    let mut buffer = vec![0u8; size];
    reader.ReadBytes(&mut buffer).unwrap();

    let base64_thumbnail = base64::encode(buffer);

    format!("data:image/png;base64,{}", base64_thumbnail)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_now_playing, get_thumbnail])
        .setup(|app| {
            let win = app.get_webview_window("main").unwrap();
            let offset_position = tauri::PhysicalPosition  {
                x: 20,
                y: 20,
            };

            let _ = win.as_ref().window().set_position(tauri::Position::Physical(offset_position));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
