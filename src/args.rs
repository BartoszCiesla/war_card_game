use clap::Parser;

#[derive(Parser)]
#[command(version)]
#[command(author)]
#[command(about = "Application simulates war card game", long_about = None)]
pub(crate) struct Args {
    /// Seed for deck shuffling
    #[arg(short, long)]
    seed: Option<u64>,
}

impl Args {
    pub(crate) fn get_seed(&self) -> Option<u64> {
        self.seed
    }
}
