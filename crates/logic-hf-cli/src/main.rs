use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};
use clap_stdin::MaybeStdin;
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(flatten)]
    verbosity: Verbosity,

    /// input file
    #[arg(id("in"), short, long)]
    in_path: MaybeStdin<Utf8PathBuf>,

    /// output file
    #[arg(id("out"), short, long)]
    out_path: Option<MaybeStdin<Utf8PathBuf>>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug)]
struct PLAArgs {}

#[derive(Subcommand, Debug)]
enum Commands {
    /// ESPRESSO PLA engine
    PLA(PLAArgs),
}

fn main() {
    let cli = Cli::parse();

    println!("verbosity: {:#?}", cli.verbosity.log_level_filter());
    match &cli.command {
        Commands::PLA(args) => {
            println!("pla args: {:#?}", args);
            println!("pla in: {:#?}", cli.in_path);
            println!("pla out: {:#?}", cli.out_path);
        }
    }
}
