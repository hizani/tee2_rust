use clap::Parser;

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

fn main() {
    let cli = Cli::parse();
}
