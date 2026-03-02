mod config;
mod init;
mod matcher;
mod opener;
mod store;

use clap::{Parser, Subcommand, ValueEnum};
use std::process;

use config::Config;
use store::{EntryType, Store};

#[derive(Parser)]
#[command(
    name = "linker",
    about = "CLI jump tool — register and jump to paths, URLs, and files"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        target: String,
    },
    Remove {
        name: String,
    },
    List,
    Edit {
        name: String,
        target: String,
    },
    Init {
        #[arg(long, value_enum, default_value = "detect")]
        shell: ShellArg,
    },
    #[command(external_subcommand)]
    Jump(Vec<String>),
}

#[derive(ValueEnum, Clone)]
enum ShellArg {
    Bash,
    Powershell,
    Detect,
}

fn main() {
    let cli = Cli::parse();
    let mut store = Store::load();
    let config = Config::load();

    match cli.command {
        Commands::Add { name, target } => match store.add(name.clone(), target.clone()) {
            Ok(()) => {
                if let Err(e) = store.save() {
                    eprintln!("Error saving bookmarks: {}", e);
                    process::exit(1);
                }
                println!("Added '{}' -> {}", name, target);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },

        Commands::Remove { name } => match store.remove(&name) {
            Ok(()) => {
                if let Err(e) = store.save() {
                    eprintln!("Error saving bookmarks: {}", e);
                    process::exit(1);
                }
                println!("Removed '{}'", name);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },

        Commands::List => {
            if store.entries.is_empty() {
                println!("No entries registered. Use 'l add <name> <target>' to add one.");
            } else {
                let width = store
                    .entries
                    .iter()
                    .map(|e| e.name.len())
                    .max()
                    .unwrap_or(0);
                for entry in &store.entries {
                    println!("{:<width$}  {}", entry.name, entry.target, width = width);
                }
            }
        }

        Commands::Edit { name, target } => match store.edit(&name, target.clone()) {
            Ok(()) => {
                if let Err(e) = store.save() {
                    eprintln!("Error saving bookmarks: {}", e);
                    process::exit(1);
                }
                println!("Updated '{}' -> {}", name, target);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },

        Commands::Init { shell } => {
            let s = match shell {
                ShellArg::Bash => init::Shell::Bash,
                ShellArg::Powershell => init::Shell::Powershell,
                ShellArg::Detect => init::Shell::detect(),
            };
            print!("{}", init::snippet(&s));
        }

        Commands::Jump(args) => {
            let name = match args.first() {
                Some(n) => n.as_str(),
                None => {
                    eprintln!("Usage: j <name>");
                    process::exit(1);
                }
            };

            match matcher::find(name, &store.entries) {
                matcher::MatchResult::None => {
                    eprintln!("No entry found matching '{}'", name);
                    process::exit(1);
                }
                matcher::MatchResult::Ambiguous(matches) => {
                    eprintln!("Ambiguous match for '{}'. Did you mean:", name);
                    for m in &matches {
                        eprintln!("  {}  ->  {}", m.name, m.target);
                    }
                    process::exit(1);
                }
                matcher::MatchResult::One(entry) => {
                    let open_explorer = args.get(1).map(|s| s == "e").unwrap_or(false);
                    let target_type = entry.target_type();
                    if target_type == EntryType::Path && open_explorer {
                        if let Err(e) = opener::open_explorer(&entry.target) {
                            eprintln!("Error opening Explorer: {}", e);
                            process::exit(1);
                        }
                    } else if target_type == EntryType::Path {
                        println!("{}", entry.target);
                        process::exit(2);
                    } else {
                        if let Err(e) = opener::open(
                            &entry.target,
                            &target_type,
                            &config,
                            entry.open_with.as_deref(),
                        ) {
                            eprintln!("Error opening '{}': {}", entry.target, e);
                            process::exit(1);
                        }
                    }
                }
            }
        }
    }
}
