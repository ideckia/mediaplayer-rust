use mpsc::{Receiver, Sender};
use rodio::{Decoder, OutputStream};
use rodio::{Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::ops::Add;
use std::sync::mpsc;
use std::time::Duration;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let arguments = rsmp::cli_args::CliArgs::parse_arguments();

    let opt_sound_path = arguments.sound_path;
    let loop_sound = arguments.loop_sound;

    match opt_sound_path {
        None => {
            show_usage();
        }
        Some(sound_path) => {
            // Load a sound from a file
            let file = File::open(&sound_path).expect("Could not load the sound file.");
            let file_buffer = BufReader::new(file);

            // Get a output stream handle to the default physical sound device
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();

            // Decode that sound file into a source
            let source = Decoder::new(file_buffer).expect("Error decoding sound file.");
            let sound_total_duration = &match source.total_duration() {
                Some(duration) => duration,
                None => {
                    println!("Could not get total duration of the sound. Getting a default duration of 5 seconds.");
                    Duration::from_secs(5)
                }
            };

            let sink = Sink::try_new(&stream_handle).unwrap();
            if loop_sound {
                sink.append(source.repeat_infinite());
            } else {
                sink.append(source);
            }

            println!(
                "Playing [{}] file (duration {}) (loop {})",
                sound_path,
                sound_total_duration.as_secs(),
                loop_sound
            );

            let (tx, rx): (Sender<Playback>, Receiver<Playback>) = mpsc::channel();

            handle_user_input(tx);

            let sleep_duration = Duration::from_millis(100);
            let mut playing_duration = Duration::from_secs(0);
            loop {
                std::thread::sleep(sleep_duration);
                match rx.try_recv() {
                    Ok(action) => match action {
                        Playback::Pause => sink.pause(),
                        Playback::Resume => sink.play(),
                        Playback::Stop => {
                            sink.stop();
                            break;
                        }
                    },
                    Err(_) => {}
                }

                if !loop_sound && !sink.is_paused() {
                    playing_duration = playing_duration.add(sleep_duration);
                    if playing_duration.ge(sound_total_duration) {
                        break;
                    }
                }
            }
        }
    }
}

fn show_usage() {
    println!("rsmp {}", VERSION);
    println!("Usage: rsmp <options>");
    println!("");
    println!("Options:");
    println!("    -p, --path <path>        path of the sound to play (accepts ogg, mp3, wav...)");
    println!("    -l, --loop               if set, sound will play in a loop");
}

fn handle_user_input(tx: Sender<Playback>) {
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        let mut user_input = String::new();
        let stop_string = String::from("stop");
        let pause_string = String::from("pause");
        let resume_string = String::from("resume");
        let sleep_duration = Duration::from_millis(100);

        loop {
            std::thread::sleep(sleep_duration);
            user_input.clear();
            match stdin.read_line(&mut user_input) {
                Ok(_) => {}
                Err(error) => println!("error reading user input: {error}"),
            }

            user_input.retain(|c| c.is_alphabetic());

            if user_input == stop_string {
                tx.send(Playback::Stop).unwrap();
                break;
            } else if user_input == pause_string {
                tx.send(Playback::Pause).unwrap();
            } else if user_input == resume_string {
                tx.send(Playback::Resume).unwrap();
            }
        }
    });
}

enum Playback {
    Pause,
    Resume,
    Stop,
}
