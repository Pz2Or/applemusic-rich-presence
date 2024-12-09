use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use dotenvy::dotenv;
use std::env;
use std::process::Command;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("Failed to load .env file");
    let discord_client_id: String =
        env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be set");

    let mut client = DiscordIpcClient::new(&discord_client_id)?;
    client.connect()?;

    let mut previous_song_info = String::new();

    loop {
        let output = Command::new("osascript")
            .arg("-e")
            .arg(r#"tell application "Music" to if player state is playing then return name of current track & " - " & artist of current track"#)
            .output()?;

        let song_info = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if song_info == previous_song_info {
            thread::sleep(Duration::from_secs(5));
            continue;
        }

        let activity = if !song_info.is_empty() {
            activity::Activity::new()
                .state("音楽を再生中")
                .details(&song_info)
                .assets(
                    activity::Assets::new()
                        .large_image("music")
                        .large_text("Apple Music"),
                )
        } else {
            activity::Activity::new()
                .state("アイドル状態")
                .details("音楽は再生されていません")
                .assets(
                    activity::Assets::new()
                        .large_image("idle")
                        .large_text("No Music Playing"),
                )
        };

        client.set_activity(activity)?;

        previous_song_info = song_info;

        thread::sleep(Duration::from_secs(5));
    }
}
