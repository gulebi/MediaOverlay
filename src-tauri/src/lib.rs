use serde::Serialize;
use windows::{Foundation::TypedEventHandler, Media::Control::{GlobalSystemMediaTransportControlsSessionManager, GlobalSystemMediaTransportControlsSessionMediaProperties}};

use std::thread;
use std::time::Duration;
use tauri::{Emitter, Manager, Window};

fn setup_media_listener(app: &mut tauri::App){
    let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .unwrap()
        .get()
        .unwrap();
    let session = manager.GetCurrentSession().unwrap();

    let app_handle = app.handle().clone();
    let session_clone = session.clone();

    session.MediaPropertiesChanged(
        &TypedEventHandler::new(move |_, _| {
            let properties = session_clone.TryGetMediaPropertiesAsync().unwrap().get().unwrap();
            let artist = properties.Artist().unwrap().to_string();
            let title = properties.Title().unwrap().to_string();

            app_handle.emit("song_changed", NowPlaying { artist, title }).unwrap();

            Ok(())
        })
    ).unwrap();
}


fn get_properties() -> GlobalSystemMediaTransportControlsSessionMediaProperties {
    let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .unwrap()
        .get()
        .unwrap();
    let session = manager.GetCurrentSession().unwrap();

    session.TryGetMediaPropertiesAsync().unwrap().get().unwrap()
}

#[derive(Debug, Serialize, Clone)]
struct NowPlaying {
    artist: String,
    title: String,
}

fn show_window(window: Window) {
    window.show().unwrap();
    let window_clone = window.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        window_clone.hide().unwrap();
    });
}

#[tauri::command]
fn get_now_playing(window: Window) -> NowPlaying {
    let properties = get_properties();

    let artist = properties.Artist().unwrap().to_string();
    let title = properties.Title().unwrap().to_string();

    show_window(window);

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
        .setup(|app: &mut tauri::App| {
            let win = app.get_webview_window("main").unwrap();
            let offset_position = tauri::PhysicalPosition  {
                x: 20,
                y: 20,
            };

            let _ = win.as_ref().window().set_position(tauri::Position::Physical(offset_position));

            setup_media_listener(app);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
