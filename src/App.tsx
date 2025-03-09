import "./App.css";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

type Song = {
    artist: string;
    title: string;
};

export default function NowPlaying() {
    const [song, setSong] = useState<Song>({ title: "Fetching...", artist: "Fetching..." });
    const [thumbnail, setThumbnail] = useState<string>("");

    const fetchNowPlaying = async () => {
        try {
            const result = await invoke<Song>("get_now_playing");
            setSong(result);
            const thumbnail = await invoke<string>("get_thumbnail");
            setThumbnail(thumbnail);
        } catch (error) {
            console.log(error);
            setSong({ title: "No media", artist: "No artist" });
        }
    };

    useEffect(() => {
        fetchNowPlaying();

        // setInterval(async () => {
        //     fetchNowPlaying();
        // }, 20000);

        const unlisten = listen<Song>("song_changed", (event) => {
            console.log(event);

            fetchNowPlaying();
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, []);

    return (
        <div className="flex items-center justify-between h-screen bg-gray-900 text-white p-2 select-none">
            <div className="flex gap-4 items-center">
                <img src={thumbnail} className="w-16 h-16 rounded-md" />
                <div className="flex flex-col gap-1 w-48">
                    <h1 className="text-md truncate">{song.title}</h1>
                    <h1 className="text-sm">{song.artist}</h1>
                </div>
            </div>
            {/* <button className="bg-gray-700 py-1 px-2 rounded-md cursor-pointer" onClick={fetchNowPlaying}>
                Rel
            </button> */}
        </div>
    );
}
