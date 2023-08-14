use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(about = "Daniel")]
#[command(version = "1.1")]
struct Cli {
    /// How long in minutes to run the pomodoro for
    #[arg(short, long)]
    time: u64,

    #[arg(short, long, action = clap::ArgAction::Count)]
    show_time: Option<u8>,

    #[clap(subcommand)]
    pub entity_type: EntType
}

#[derive(Debug, Args)]
struct OneStruct {
    param1: u64
}

#[derive(Debug, Args)]
struct TwoStruct {
    param1: u64,
    param2: u64
}

#[derive(Subcommand, Debug)]
enum EntType {
    One(OneStruct),
    Two(TwoStruct)
}



fn main() {
    let cli = Cli::parse();
    let t = cli.time;
 
}