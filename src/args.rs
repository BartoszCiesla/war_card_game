use clap::{Parser, ValueEnum};

#[derive(Default, ValueEnum, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) enum Output {
    OneLine,
    Quiet,
    #[default]
    Normal,
    Verbose,
}

#[derive(Parser)]
#[command(version)]
#[command(author)]
#[command(about = "Application simulates war card game", long_about = None)]
pub(crate) struct Args {
    /// Seed for deck shuffling
    #[arg(short, long)]
    seed: Option<u64>,

    /// Output level
    #[arg(long, short = 'o', default_value = "normal", group = "output_mode")]
    output: Output,

    /// Verbose mode
    #[arg(long, short = 'v', default_value = "false", group = "output_mode")]
    verbose: bool,

    /// Quiet mode
    #[arg(long, short = 'q', default_value = "false", group = "output_mode")]
    quiet: bool,

    /// One line output mode
    #[arg(long, short = '1', default_value = "false", group = "output_mode")]
    one_line: bool,
}

impl Args {
    pub(crate) fn get_seed(&self) -> Option<u64> {
        self.seed
    }

    pub(crate) fn get_output(&self) -> Output {
        if self.one_line {
            Output::OneLine
        } else if self.quiet {
            Output::Quiet
        } else if self.verbose {
            Output::Verbose
        } else {
            self.output.clone()
        }
    }
}
