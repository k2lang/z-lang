use clap::{Parser, Subcommand};
use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;
use z_lang::{compile_file, run_file};

#[derive(Parser)]
#[command(name = "zc")]
#[command(about = "Z Programming Language Compiler", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Z source file to an executable
    Compile {
        /// Input file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Output file
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Optimization level (0-3)
        #[arg(short, long, default_value_t = 3)]
        opt_level: u8,
    },
    /// Run a Z source file directly
    Run {
        /// Input file
        #[arg(value_name = "FILE")]
        input: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile {
            input,
            output,
            opt_level,
        } => {
            let output = output.unwrap_or_else(|| {
                let mut out = input.file_stem().unwrap().to_owned();
                out.to_string_lossy().to_string().into()
            });
            
            println!("Compiling {} to {} with optimization level {}", 
                input.display(), output.display(), opt_level);
                
            compile_file(&input, &output, opt_level).into_diagnostic()?;
            println!("Compilation successful!");
        }
        Commands::Run { input } => {
            println!("Running {}", input.display());
            run_file(&input).into_diagnostic()?;
        }
    }

    Ok(())
}