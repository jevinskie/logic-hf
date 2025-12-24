use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};
use clap_stdin::MaybeStdin;
use clap_verbosity_flag::Verbosity;
use std::io::Write;

// #[derive(Parser, Debug)]
// #[command(version, about)]
// struct Cli {
//     #[command(flatten)]
//     verbosity: Verbosity,

//     /// input file
//     #[arg(id("in"), short, long)]
//     in_path: MaybeStdin<Utf8PathBuf>,

//     /// output file
//     #[arg(id("out"), short, long)]
//     out_path: Option<MaybeStdin<Utf8PathBuf>>,

//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Args, Debug)]
// struct PlaArgs {}

// #[derive(Subcommand, Debug)]
// enum Commands {
//     /// ESPRESSO PLA engine
//     Pla(PlaArgs),
// }

// fn main() {
//     let cli = Cli::parse();

//     println!("verbosity: {:#?}", cli.verbosity.log_level_filter());
//     match &cli.command {
//         Commands::Pla(args) => {
//             println!("pla args: {:#?}", args);
//             println!("pla in: {:#?}", cli.in_path);
//             println!("pla out: {:#?}", cli.out_path);
//         }
//     }
// }

fn main() -> Result<(), String> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

fn respond(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;
    match cli.command {
        Commands::Ping => {
            write!(std::io::stdout(), "Pong").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Commands::Exit => {
            write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            return Ok(true);
        }
    }
    Ok(false)
}

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Ping,
    Exit,
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "$ ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
