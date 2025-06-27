pub mod chunk;
pub mod chunk_type;
pub mod png;

pub use chunk::Chunk;
pub use chunk_type::ChunkType;
pub use png::Png;

use std::path::PathBuf;
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
use std::{str::FromStr};


pub fn hide_message(chunk_type: &str, message: &str, file_path: PathBuf, output_file: Option<PathBuf>) -> Result<()>
{
    let png_data = std::fs::read(&file_path)?;
    let png_bytes: &[u8] = &png_data;
    let mut png = Png::try_from(png_bytes)?;
    let data = message.as_bytes().to_vec();
    let chunk_type_struct = ChunkType::from_str(&chunk_type)?;
    let new_chunk = Chunk::new(chunk_type_struct, data);
    png.append_chunk(new_chunk);
    if let Some(save_path) = output_file {
        std::fs::write(save_path, png.as_bytes())?;
    }
    else{
        std::fs::write(file_path, png.as_bytes())?;
    }
    Ok(())
}

pub fn extract_message(file_path: PathBuf, chunk_type: &str) -> Result<String>
{
    let png_data = std::fs::read(file_path)?;
    let png_bytes: &[u8] = &png_data;
    let png = Png::try_from(png_bytes)?;
    if let Some(chunk) = png.chunk_by_type(&chunk_type){
        let data = chunk.data_as_string()?;
        Ok(data)
    }
    else{
        Err("Could not find message for chunk type".into())
    }
}

pub fn remove_chunk(file_path: PathBuf, chunk_type: &str) -> Result<()>
{
    let png_data = std::fs::read(&file_path)?;
    let png_bytes: &[u8] = &png_data;
    let mut png = Png::try_from(png_bytes)?;
    png.remove_first_chunk(&chunk_type)?;
    std::fs::write(file_path, png.as_bytes())?;
    Ok(())
}

pub fn list_chunks(file_path: PathBuf) -> Result<Vec<(String, u32)>>
{
    let png_data: Vec<u8> = std::fs::read(file_path)?;
    let png_bytes: &[u8] = &png_data;
    let png = Png::try_from(png_bytes)?;
    let png_chunks = png.chunks();
    let mut res = Vec::new();
    for chunk in png_chunks.iter(){
        let chunk_info = (chunk.chunk_type.to_string(), chunk.length());
        res.push(chunk_info);
    }
    Ok(res)
}