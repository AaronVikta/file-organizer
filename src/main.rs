use clap::{Parser, Subcommand};

#[derive(Debug,Parser)]
#[command(name = "File Organizer",version="1.0")]
struct  Cli{
    // Source directory to organize
    #[command(subcommand)]
    command: Actions

}

#[derive(Debug, Subcommand)]
enum Actions{
    ORGANIZE{
        /// Source directory to organize
        #[arg(short, long)]
        source: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command{
        Actions::ORGANIZE{source}=> {
            println!("Organizing files in directory: {}", source);
            // Add file organization logic here
        }
    }
}
