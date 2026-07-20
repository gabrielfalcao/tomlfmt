use clap::Parser;
use color_eyre::Report;
use iocore::Path;
use tomlfmt::prettify_file;

#[derive(Parser)]
#[command(version, about, long_about = "simple and fast toml prettifier")]
struct Cli {
    #[arg(required = true, help = "list of toml paths to prettify")]
    paths: Vec<Path>,

    #[arg(short, long, help = "write toml files instead of printing to stdout")]
    write_in_place: bool,
}

fn main() -> Result<(), Report> {
    let args = Cli::parse();
    for path in args.paths.iter() {
        prettify_file(path, args.write_in_place)?;
    }
    Ok(())
}
