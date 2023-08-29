// TODO: Simple timer that sleeps and ends -> COMPLETE
// TODO: Timer that makes a sound when it ends -> COMPLETE
// TODO: Timer that takes a cmd line param for how long it runs -> COMPLETE
// TODO: Progress bar and countdown -> COMPLETE

// TODO: Ability to clear screen or not clear screen as param
// TODO: Ability to pause/resume


use std::time::Duration;
use std::{fs::File, error::Error};
use std::io::BufReader;
use rodio::{OutputStream, Sink, Decoder};
use indicatif::{ProgressBar, ProgressStyle, FormattedDuration};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    ///Number of minutes to run the pomodoro for
    #[arg(short, long)]
    minutes: u32
}

#[test]
fn test_play_sound() {
    play_sound(None)
}

fn play_sound(path: Option<&str>) {
    //Default sound path
    let path = path.unwrap_or("/Users/danielseisun/workspace/pomodoro/ding.mp3");
    let file = BufReader::new(File::open(path).unwrap());
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = Decoder::new(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}

fn countdown(time_in_minutes: u32) {
    
    let time_in_secs = time_in_minutes * 60;
    
    let style = ProgressStyle::with_template("{spinner} {elapsed} {percent}% {msg} {bar:40.cyan/blue}")
        .unwrap();
    let bar = ProgressBar::new(time_in_secs.into()).with_style(style);
    let tick = Duration::from_secs(1);

    std::process::Command::new("clear").status().unwrap();
    for secs_elapsed in 0..time_in_secs {




        let time_remaining = time_in_secs - secs_elapsed;
        bar.set_message(format!("Time remaining: {}", FormattedDuration(Duration::from_secs(time_remaining.into()))));
        bar.inc(1);
        // let time_left = Duration::from_secs(time_remaining.into());
        std::thread::sleep(tick);
    }
    


}


fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    countdown(cli.minutes);
    play_sound(None);
    println!("Completed pomodoro. {} minutes elapsed", cli.minutes);
    Ok(())

}
