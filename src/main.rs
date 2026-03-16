mod chive;
mod cmd;
mod color;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "chivest", version, about = "Directory file tracker and annotator")]
struct Cli {
    /// Pacman-style sync: alias for 'update' (combine with -y to auto-confirm)
    #[arg(short = 'S', global = false)]
    sync: bool,
    /// Auto-confirm removals when used with -S
    #[arg(short = 'y', global = false)]
    yes: bool,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new .chivest file for the current directory
    New {
        /// Include hidden files (same as -la)
        #[arg(short = 'a', long = "all")]
        all: bool,
        /// Combined -la style (also includes hidden files)
        #[arg(short = 'l', hide = true)]
        long_flag: bool,
    },
    /// Display the chivest table
    Ls {
        /// Show additional metadata (date last edited, created date)
        #[arg(short = 'l', long = "long")]
        long: bool,
        /// Combined -la style
        #[arg(short = 'a', hide = true)]
        all_flag: bool,
    },
    /// Edit description and notes for an item
    Edit {
        /// Item number to edit (prompted if omitted)
        item: Option<usize>,
    },
    /// Search entries by filename, description, or notes
    Find {
        /// Search query (case-insensitive; multi-word queries can be quoted)
        #[arg(required = true, num_args = 1..)]
        query: Vec<String>,
    },
    /// Clear description/notes for an item, or remove the .chivest file itself
    Rm {
        /// Remove the entire .chivest file
        #[arg(long = "chive", short = 'C')]
        chive: bool,
        /// Item number whose description should be cleared (prompted if omitted)
        item: Option<usize>,
    },
    /// Sync .chivest with the current directory (add new, prompt for removed)
    Update {
        /// Include hidden files
        #[arg(short = 'a', long = "all")]
        all: bool,
        /// Auto-confirm removal of missing entries
        #[arg(short = 'y')]
        yes: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // Top-level -S flag is a pacman-style alias for 'update'
    let result = if cli.sync {
        cmd::update::run(false, cli.yes)
    } else {
        match cli.command {
            Some(Command::New { all, long_flag })   => cmd::new::run(all || long_flag),
            Some(Command::Ls { long, .. })           => cmd::ls::run(long),
            Some(Command::Edit { item })             => cmd::edit::run(item),
            Some(Command::Find { query })            => cmd::find::run(&query.join(" ")),
            Some(Command::Rm { chive, item })        => cmd::rm::run(chive, item),
            Some(Command::Update { all, yes })       => cmd::update::run(all, yes),
            None => {
                eprintln!("No command specified. Run 'chivest --help' for usage.");
                std::process::exit(1);
            }
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
