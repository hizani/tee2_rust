use clap::Parser;
use std::io::{Read, Write};
use std::{fs::OpenOptions, io};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Append to the given FILEs, do not overwrite
    #[clap(short, long)]
    append: bool,

    /// Name of the FILEs to write to
    #[clap(value_parser, value_name = "FILE")]
    files: Vec<String>,
}

fn tee(append_flag: bool, filename: &str, input: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true) // open or create
        .write(true) // with write mode
        .append(append_flag) // append if append flag is true
        .truncate(!append_flag) // truncate if append flag is false
        .open(filename)?;

    write!(file, "{}", input)?;

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    // read standart input to the buffer
    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    for filename in &cli.files {
        if let Err(e) = tee(cli.append, filename, &buf) {
            eprintln!("{}: {}", filename, e);
        }
    }

    print!("{}", buf);
}
