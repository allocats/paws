use clap::Parser;

mod args;
mod notes;

use crate::args::{Args, Commands};
use crate::notes::*;

fn main() {
    let args = Args::parse();
    let mut notes = load_notes();
 
    match args.command {
        Some(Commands::Add { note }) => {
            match notes.add_note(note, args.global) {
                Ok(()) => {
                    save_notes(&notes).expect("Failed to save notes");
                    println!("Note added!");
                }
                Err(e) => println!("Error adding note: {}", e),
            }
        }
        Some(Commands::Remove { id }) => {
            match notes.remove_note(id, args.global) {
                Ok(_) => {
                    save_notes(&notes).expect("Failed to save notes");
                    println!("Note removed");
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        None => {
            if args.global {
                print_global_notes(&notes);
            } else {
                print_cwd_notes(&notes);
            }
        }
    }
}
