use livekit::{track::RemoteTrack, webrtc::audio_stream::native::NativeAudioStream, Room, RoomEvent, RoomOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = std::env::var("LIVEKIT_URL")?;
    let token = std::env::var("LIVEKIT_TOKEN")?;
    let options = RoomOptions::default();

    let (room, mut events) = Room::connect(&url, &token, options).await?;

    loop {
        if let Some(event) = events.recv().await {
            match event {
                RoomEvent::TrackPublished {
                    publication,
                    participant,
                } => {


                },
                RoomEvent::TrackSubscribed { track, publication, participant } => {
                    match track {
                        RemoteTrack::Audio(track) => {
                            let sample_rate = 48000;
                            let channels = 2;

                           //NativeAudioStream::new(track, sample_rate, ) track.
                        },
                        _ => {},
                    }

                }
                _ => {}
            }
        }
    }
}
