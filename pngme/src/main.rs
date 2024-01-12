mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::Parser;
use args::PngMeArgs;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: PngMeArgs
}


fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        PngMeArgs::Encode(encode_args) => {
            println!("{:?},{:?},{:?},{:?}", encode_args.file_path, encode_args.chunk_type, encode_args.message, encode_args.output_file);
        },
        PngMeArgs::Decode(decode_args) => {
            println!("{:?},{:?}", decode_args.file_path, decode_args.chunk_type);
        },
        PngMeArgs::Remove(remove_args) => {
            println!("{:?},{:?}", remove_args.file_path, remove_args.chunk_type);
        },
        PngMeArgs::Print(print_args) => {
            println!("{:?}", print_args.file_path);
        },
    }

    Ok(())
}

