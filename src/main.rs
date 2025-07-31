use livekit::{Room, RoomOptions};

async fn main() -> anyhow::Result {
    let url = std::env!("LIVEKIT_URL");
    let token = std::env!("LIVEKIT_TOKEN");
    let options = RoomOptions::default();

    let (room, events) = Room::connect(url, token, options).await?;

    if let Some(event: RoomEvent) = events.recv() {
        
    }
}
