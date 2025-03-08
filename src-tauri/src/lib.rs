use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;
use serde::Serialize;

fn get_properties() -> windows::Media::Control::GlobalSystemMediaTransportControlsSessionMediaProperties {
    let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap();
    let session = manager.GetCurrentSession().unwrap();
    session.TryGetMediaPropertiesAsync().unwrap().get().unwrap()
}

#[derive(Debug, Serialize)]
struct NowPlaying {
    artist: String,
    title: String,
}

#[tauri::command]
fn get_now_playing() -> NowPlaying {
    let properties = get_properties();

    let artist = properties.Artist().unwrap().to_string();
    let title = properties.Title().unwrap().to_string();

    NowPlaying { artist, title }
}

#[tauri::command]
fn get_thumbnail() -> String {
   "foo".to_string()
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_now_playing, get_thumbnail])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
