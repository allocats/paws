use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long, help="Show global notes")]
    pub global: bool,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    Add {note: String},
    Remove {id: i32},
}
