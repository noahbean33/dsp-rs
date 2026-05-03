use clap::Parser;

/// Rebuilding core OS system utilities in Rust.
#[derive(Debug, Parser)]
#[command(name = "os-core-utils", version, about, long_about = None)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        if self.verbose {
            println!("Verbose mode enabled");
        }
        println!("os-core-utils v{}", env!("CARGO_PKG_VERSION"));
        Ok(())
    }
}
