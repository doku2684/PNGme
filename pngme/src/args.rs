use clap::{Subcommand, Args};
use std::path::PathBuf;
use crate::chunk_type::ChunkType;


#[derive(Subcommand, Debug, Clone)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Clone, Debug, Args)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
    pub message: String,
    pub output_file: Option<PathBuf>,
}

#[derive(Clone, Debug, Args)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(Clone, Debug, Args)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(Clone, Debug, Args)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}
