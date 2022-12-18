mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use crate::args::Commands;
use crate::commands::{decode, encode, print_chunks, remove};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add(name) => {
            println!("{:?}", name.name);
        }
        Commands::Encode(encode_args) => {
            println!(
                "{:?}{}{}{:?}",
                encode_args.file_path,
                encode_args.chunk_type,
                encode_args.message,
                encode_args.output_file
            );
            encode(encode_args.clone())?;
        }
        Commands::Decode(decode_args) => {
            println!("{:?}{}", decode_args.file_path, decode_args.chunk_type,);
            decode(decode_args.clone())?;
        }
        Commands::Remove(remove_args) => {
            println!("{:?}{}", remove_args.file_path, remove_args.chunk_type,);
            remove(remove_args.clone())?;
        }
        Commands::Print(print_args) => {
            println!("{:?}", print_args.file_path,);
            print_chunks(print_args.clone())?;
        }
    }
    Ok(())
}
