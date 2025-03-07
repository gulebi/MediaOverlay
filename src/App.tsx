import "./App.css";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function NowPlaying() {
    const [song, setSong] = useState("Fetching...");

    const fetchNowPlaying = async () => {
        try {
            const result = await invoke<string>("get_now_playing");
            setSong(result);
            const thumbnail = await invoke<string>("get_thumbnail");
            console.log(thumbnail);
        } catch (error) {
            console.log(error);
            setSong("No media playing");
        }
    };

    useEffect(() => {
        fetchNowPlaying();
    }, []);

    return (
        <div className="flex flex-col items-center justify-center h-screen bg-gray-900 text-white p-4 gap-2">
            <h1 className="text-xl text-center">{song}</h1>
            <button className="bg-gray-700 py-1 px-2 rounded-md cursor-pointer" onClick={fetchNowPlaying}>
                Refresh
            </button>
        </div>
    );
}
