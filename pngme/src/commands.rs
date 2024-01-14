use crate::args::{PngMeArgs, EncodeArgs, DecodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::png::Png;
use clap::Parser;
use std::fs;
use std::convert::TryFrom;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: PngMeArgs
}

pub fn execute_command() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        PngMeArgs::Encode(EncodeArgs{file_path, chunk_type, message, output_file}) => {
            let mut png = Png::try_from(fs::read(&file_path)?.as_slice()).unwrap();

            let chunk = Chunk::new(chunk_type, message.into());

            png.append_chunk(chunk);
            
            if let Some(output_file) = output_file {
                fs::write(output_file, png.as_bytes())?;
            } else {
                fs::write(file_path, png.as_bytes())?;
            }

            Ok(())
        },
        PngMeArgs::Decode(DecodeArgs{file_path, chunk_type}) => {
            let png = Png::try_from(fs::read(file_path)?.as_slice()).unwrap();

            if let Some(chunk) = png.chunk_by_type(format!("{}", chunk_type).as_str()) {
                println!("{}", chunk.data_as_string()?);
                Ok(())
            } else {
                Err("chunk does not exist".into())
            }
        },
        PngMeArgs::Remove(RemoveArgs{file_path, chunk_type}) => {
            let mut png = Png::try_from(fs::read(&file_path)?.as_slice()).unwrap();

            png.remove_chunk(format!("{}", chunk_type).as_str())?;

            fs::write(file_path, png.as_bytes())?;

            Ok(())
        },
        PngMeArgs::Print(PrintArgs{file_path}) => {
            println!("{:?}", Png::try_from(fs::read(file_path)?.as_slice()).unwrap());
            Ok(())
        },
    }
}
