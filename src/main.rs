#![allow(unused_imports)]
#![allow(dead_code)]
pub mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use png::Png;
use chunk::Chunk;
use chunk_type::ChunkType;
use clap::Parser;
use std::{fs, str::FromStr};
use args::{Commands, Args};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
use pngme::{hide_message, extract_message, remove_chunk, list_chunks};

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Encode { file_path, chunk_type, message, output_file } => {
            hide_message(&chunk_type, &message, file_path, output_file)?;
            println!("Encoding");
        },
        Commands::Decode { file_path, chunk_type } => {
            let message = extract_message(file_path, &chunk_type)?;
            println!("Found chunk data : {}", message);
            println!("Decoding");
        },
        Commands::Remove { file_path, chunk_type } => {
            remove_chunk(file_path, &chunk_type)?;
            println!("Removing");
        },
        Commands::Print { file_path } => {
            let chunks = list_chunks(file_path)?;
            for (chunk_type, length) in chunks {
                println!("Chunk Type: {}, Length: {}", chunk_type, length);
            }
        }
    }
    Ok(())
}