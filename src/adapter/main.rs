trait MediaPlayerTrait {
    fn play(&self, t: Type);
}

trait AdvancedMediaPlayerTrait {
    fn play_vlc(&self);
    fn play_mp4(&self);
}

struct VlcPlayer;
struct Mp4Player;

impl AdvancedMediaPlayerTrait for VlcPlayer {
    fn play_vlc(&self) {
        println!("vlc players play vlc video");
    }

    fn play_mp4(&self) {
        println!("vlc player cannot play mp4");
    }
}

impl AdvancedMediaPlayerTrait for Mp4Player {
    fn play_vlc(&self) {
        println!("mp4 player cannot play vlc video");
    }

    fn play_mp4(&self) {
        println!("mp4 players play mp4");
    }
}
/// The following two structs illustrate the pattern
struct MediaAdapter;
struct AudioPlayer {
    player: MediaAdapter,
}

#[derive(Debug)]
enum Type {
    MP3,
    MP4,
    VLC,
}
impl MediaPlayerTrait for MediaAdapter {
    fn play(&self, t: Type) {
        match t {
            Type::MP4 => Mp4Player.play_mp4(),
            Type::VLC => VlcPlayer.play_vlc(),
            _ => println!("{:?} not supported", t),
        }
    }
}

impl AudioPlayer {
    fn new() -> Self {
        Self {
            player: MediaAdapter,
        }
    }
    fn play_mp3(&self) {
        println!("audio players play mp3");
    }
}

impl MediaPlayerTrait for AudioPlayer {
    fn play(&self, t: Type) {
        match t {
            Type::MP3 => self.play_mp3(),
            _ => self.player.play(t),
        }
    }
}

fn main() {
    let audio_player = AudioPlayer::new();
    audio_player.play(Type::MP3);
    audio_player.play(Type::MP4);
    audio_player.play(Type::VLC);
}
