
use clap::{Parser, Subcommand};
use crate::chunk_type::ChunkType;
use std::path::PathBuf;


#[derive(Parser)]
#[command(name = "pngme")]
#[command(about = "A PNG message encoder/decoder")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode{
        file_path: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },
    Decode{
        file_path: PathBuf,
        chunk_type: String,
    },
    Remove{
        file_path: PathBuf,
        chunk_type: String,
    },
    Print{
        file_path: PathBuf,
    }
}