use clap::Parser;
use color_eyre::Report;
use iocore::Path;
use tomlfmt::parse_source;

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
        let source = path.read()?;
        let parsed = parse_source(&source)?;
        eprintln!("{parsed:#?}");
    }
    Ok(())
}
