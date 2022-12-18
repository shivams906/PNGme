use clap::{Args, Subcommand};
use std::path::PathBuf;
#[derive(Subcommand)]
pub enum Commands {
    Add(Add),
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

#[derive(Args)]
pub struct Add {
    pub name: Option<String>,
}

#[derive(Args, Clone)]
pub struct Encode {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<PathBuf>,
}

#[derive(Args, Clone)]
pub struct Decode {
    pub file_path: PathBuf,
    pub chunk_type: String,
}
#[derive(Args, Clone)]
pub struct Remove {
    pub file_path: PathBuf,
    pub chunk_type: String,
}
#[derive(Args, Clone)]
pub struct Print {
    pub file_path: PathBuf,
}
