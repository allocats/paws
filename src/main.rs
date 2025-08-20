use clap::error::Result;
use clap::Parser;
use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, help="Show global notes")]
    global: bool,
}

#[derive(clap::Subcommand)]
enum Commands {
    Add {note: String},
    Remove {id: i32},
}

#[derive(Serialize, Deserialize, Default)]
struct NotesData {
    notes: HashMap<String, DirectoryNotes>
}

#[derive(Serialize, Deserialize)]
struct DirectoryNotes {
    count: i32,
    notes: HashMap<i32, String>
}

impl NotesData {
    pub fn add_note(&mut self, note: String, global: bool) -> Result<(), Box<dyn std::error::Error>> {
        let dir_key = if global {
            "global".to_string()
        } else {
            std::env::current_dir()?.to_string_lossy().to_string()
        };

        let dir_notes = self.notes.entry(dir_key).or_insert_with(|| DirectoryNotes {
            count: 1,
            notes: HashMap::new(),
        });
        
        dir_notes.notes.insert(dir_notes.count as i32, note);
        dir_notes.count += 1;

        Ok(())
    }

    pub fn remove_note(&mut self, id: i32, global: bool) -> Result<(), String> {
        let dir_key = if global {
            "global".to_string()
        } else {
            std::env::current_dir().map_err(|_| "Could not get cwd")?.to_string_lossy().to_string()
        };

        match self.notes.get_mut(&dir_key) {
            Some(dir_notes) => {
                dir_notes.notes.remove(&id).ok_or_else(|| format!("Note with id {} not found", id))?;
                Ok(())
            },
            None => Err("No notes found".to_string())
        }
    }
}

fn get_notes_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME env var not set!");
    PathBuf::from(home).join(".notes.json")
}

fn load_notes() -> NotesData {
    let path = get_notes_path();

    match fs::read_to_string(&path) {
        Ok(contents) => {
            serde_json::from_str(&contents).unwrap_or_default()
        }

        Err(_) => NotesData::default()
    }
}

fn save_notes(notes: &NotesData) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(notes)?;
    let path = get_notes_path();
    fs::write(path, json)?;
    Ok(())
}

fn print_cwd_notes(notes: &NotesData) {
    let cwd = std::env::current_dir()
        .expect("Unable to read current directory")
        .to_string_lossy()
        .to_string();

    match notes.notes.get(&cwd) {
        Some(dir_notes) => {
            if dir_notes.notes.is_empty() {
                println!("No notes for this directory");
            } else {
                println!("Notes for {}:", cwd);

                for (id, note) in &dir_notes.notes {
                    println!("\t{}: {}", id, note);
                }
            }
        }

        None => println!("No notes for this directory"),
    }
}

fn print_global_notes(notes: &NotesData) {
    match notes.notes.get("global") {
        Some(dir_notes) => {
            if dir_notes.notes.is_empty() {
                println!("No global notes");
            } else {
                println!("Global notes:");

                for (id, note) in &dir_notes.notes {
                    println!("\t{}: {}", id, note);
                }
            }
        }

        None => println!("No global notes"),
    }
}

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
        },
        Some(Commands::Remove { id }) => {
            match notes.remove_note(id, args.global) {
                Ok(_) => {
                    save_notes(&notes).expect("Failed to save notes");
                    println!("Note removed");
                }
                Err(e) => println!("Error: {}", e),
            }
        },
       
        None => {
            if args.global {
                print_global_notes(&notes);
            } else {
                print_cwd_notes(&notes);
            }
        }
    }
}
